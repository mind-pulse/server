// 自动合并的实现（替代 pascalgn/automerge-action）。
//
// 由两处调用：
//   - pr-check.yml：本工作流的检查跑完后直接合并（括号里的检查就是刚跑完的，无需再查）
//   - automerge-label.yml：给 PR 打上 automerge 标签时触发——pull_request 事件的默认
//     类型不含 labeled，只有单独一个工作流才能响应打标签；打标签通常早于 PR Check
//     完成，所以这里用 waitForChecks 等检查跑完再判断
//
// 行为：
//   - 只处理 open、非 draft、带 automerge 标签的 PR
//   - 合并方式为 merge（与之前一致）；合并成功后移除 automerge 标签
//   - 失败只告警、不让作业失败：自动合并失败不该把 PR 检查标红
//
// 已知限制：改动 .github/workflows/** 的 PR 不能用 GITHUB_TOKEN 合并——GitHub 不允许
// 没有 workflows 权限的 App 修改工作流文件（API 会明确拒绝，之前 automerge-action
// 重试 6 次后失败就是这个原因）。配置 AUTOMERGE_TOKEN（fine-grained PAT，Contents /
// Pull requests / Workflows 均为 Read and write）即可合并这类 PR。

const LABEL = "automerge";

/** 本工作流自身的 check-run 名字：查检查结果时要排除，否则会把自己判成"未完成" */
const OWN_CHECK_NAMES = new Set(["Auto Merge PR", "Auto Merge (label)"]);

/** 视为"通过"的结论：跳过与中立的检查不该阻止合并 */
const PASSING_CONCLUSIONS = new Set(["success", "neutral", "skipped"]);

/** 等待检查完成时的轮询间隔 */
const POLL_INTERVAL_MS = 20 * 1000;

async function merge({ github, context, core, verifyChecks }) {
  const pr = context.payload.pull_request;
  if (!pr) {
    core.info("不是 pull_request 事件，跳过");
    return;
  }

  const skip = describeSkip(pr);
  if (skip) {
    core.info(`PR #${pr.number} ${skip}，跳过`);
    return;
  }

  const owner = context.repo.owner;
  const repo = context.repo.repo;

  if (
    verifyChecks &&
    !(await checksPassed({ github, core, owner, repo, pr }))
  ) {
    return;
  }

  try {
    await github.rest.pulls.merge({
      owner,
      repo,
      pull_number: pr.number,
      merge_method: "merge",
    });
  } catch (error) {
    core.warning(`PR #${pr.number} 自动合并失败：${explainMergeError(error)}`);
    return;
  }

  core.info(`已合并 PR #${pr.number}`);

  try {
    await github.rest.issues.removeLabel({
      owner,
      repo,
      issue_number: pr.number,
      name: LABEL,
    });
  } catch (error) {
    // 标签删不掉不影响合并结果
    core.info(`移除 ${LABEL} 标签失败（忽略）：${error?.message ?? error}`);
  }
}

/** 返回跳过原因；可以合并时返回 null。 */
function describeSkip(pr) {
  const labels = (pr.labels ?? []).map((label) => label.name);
  if (!labels.includes(LABEL)) return `没有 ${LABEL} 标签`;
  if (pr.state !== "open") return `状态为 ${pr.state}`;
  if (pr.draft) return "仍是草稿";
  return null;
}

async function checksPassed({ github, core, owner, repo, pr }) {
  let data;
  try {
    ({ data } = await github.rest.checks.listForRef({
      owner,
      repo,
      ref: pr.head.sha,
      per_page: 100,
    }));
  } catch (error) {
    // 查询失败（典型是 token 缺少 Checks: read）不能把作业炸掉，也不能贸然合并。
    // 这一查询用默认 GITHUB_TOKEN 就够（它自带 checks: read），
    // 只有真正合并那一步才需要 AUTOMERGE_TOKEN。
    core.warning(
      `PR #${pr.number} 读取检查结果失败，跳过自动合并：${String(error?.message ?? error)}` +
        "（请确认该步骤使用的 token 具备 Checks: read）",
    );
    return false;
  }

  const runs = data.check_runs.filter((run) => !OWN_CHECK_NAMES.has(run.name));

  if (runs.length === 0) {
    core.info(`PR #${pr.number} 没有任何检查项，跳过自动合并`);
    return false;
  }

  const pending = runs.filter((run) => run.status !== "completed");
  if (pending.length > 0) {
    core.info(
      `PR #${pr.number} 仍有检查未完成：${pending.map((run) => run.name).join("、")}` +
        "；等检查结束后重新打上标签即可",
    );
    return false;
  }

  const failed = runs.filter((run) => !PASSING_CONCLUSIONS.has(run.conclusion));
  if (failed.length > 0) {
    core.warning(
      `PR #${pr.number} 有检查未通过：${failed
        .map((run) => `${run.name} = ${run.conclusion}`)
        .join("、")}`,
    );
    return false;
  }

  return true;
}

/** 把 GitHub 的合并失败信息翻译成可操作的建议。 */
function explainMergeError(error) {
  const message = String(error?.message ?? error);

  if (/workflows?[\s\S]*permission/i.test(message)) {
    return (
      "该 PR 改动了 .github/workflows/**，而 GITHUB_TOKEN 没有 workflows 权限（这是 GitHub 的硬限制）。" +
      "请手动合并，或配置 AUTOMERGE_TOKEN（fine-grained PAT，Contents / Pull requests / Workflows 均为 Read and write）"
    );
  }

  return message;
}

/**
 * 等待该 PR 的检查全部完成（最多 timeoutMs）。
 *
 * 与 checksPassed 的区别：检查未完成（或尚未创建）时会轮询等待，而不是直接
 * 判定失败。给 PR 打标签通常发生在 PR Check 跑完之前，直接判失败会永远错过
 * 合并；这里等它跑完再判断。
 *
 * 返回 true 表示检查全部通过；失败、读取错误或超时都返回 false。
 */
async function waitForChecks({
  github,
  core,
  owner,
  repo,
  pr,
  timeoutMs = 30 * 60 * 1000,
}) {
  const deadline = Date.now() + timeoutMs;
  let pendingNames = [];

  while (true) {
    let runs;

    try {
      ({ data: { check_runs: runs } } = await github.rest.checks.listForRef({
        owner,
        repo,
        ref: pr.head.sha,
        per_page: 100,
      }));
    } catch (error) {
      core.warning(
        "PR #" + pr.number + " 读取检查结果失败，跳过自动合并：" +
          String(error?.message ?? error) +
          "（请确认该步骤使用的 token 具备 Checks: read）",
      );
      return false;
    }

    const relevant = runs.filter((run) => !OWN_CHECK_NAMES.has(run.name));

    if (relevant.length > 0) {
      const pending = relevant.filter((run) => run.status !== "completed");

      if (pending.length === 0) {
        const failed = relevant.filter(
          (run) => !PASSING_CONCLUSIONS.has(run.conclusion),
        );

        if (failed.length > 0) {
          core.warning(
            "PR #" + pr.number + " 有检查未通过：" +
              failed.map((run) => run.name + " = " + run.conclusion).join("、"),
          );
          return false;
        }

        return true;
      }

      pendingNames = pending.map((run) => run.name);
    } else {
      // 检查尚未创建（工作流可能还没开始），继续等
      pendingNames = [];
    }

    if (Date.now() >= deadline) {
      core.warning(
        "PR #" + pr.number + " 等待检查超时（" +
          Math.round(timeoutMs / 60000) +
          " 分钟）" +
          (pendingNames.length ? "，仍未完成：" + pendingNames.join("、") : "") +
          "；跳过自动合并",
      );
      return false;
    }

    core.info(
      "PR #" + pr.number + " 检查尚未完成" +
        (pendingNames.length
          ? "：" + pendingNames.join("、")
          : "（尚未创建）") +
        "，" +
        POLL_INTERVAL_MS / 1000 +
        "s 后重试",
    );

    await new Promise((resolve) => setTimeout(resolve, POLL_INTERVAL_MS));
  }
}

module.exports = {
  merge,
  describeSkip,
  explainMergeError,
  checksPassed,
  waitForChecks,
};
