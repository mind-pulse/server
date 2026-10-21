#!/usr/bin/env python3
"""按 bump_type + release_channel 递增 Cargo 版本，并输出 VERSION。

语义与 npm version 对齐：
- stable 通道：patch / minor / major，丢弃预发布后缀；
- alpha/beta 通道：prerelease 只递增预发布号；patch/minor/major 先递增基础版本，
  再进入 <channel>.0。
"""

import os
import re
import sys
from pathlib import Path


def fail(message):
    print("::error::" + message)
    sys.exit(1)


bump = os.environ["BUMP_TYPE"]
channel = os.environ["RELEASE_CHANNEL"]

toml = Path("Cargo.toml")
text = toml.read_text()
match = re.search(r'^version = "([^"]+)"', text, re.M)
if not match:
    fail("Cargo.toml 中找不到 version 字段")
current = match.group(1)

m = re.fullmatch(r"(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z.-]+))?", current)
if not m:
    fail("无法解析版本号：" + current)
major, minor, patch = int(m.group(1)), int(m.group(2)), int(m.group(3))
pre = m.group(4)


def fmt(maj, mi, pa, pre_value):
    base = str(maj) + "." + str(mi) + "." + str(pa)
    return base + ("-" + pre_value if pre_value else "")


if channel == "stable":
    if bump == "prerelease":
        fail("bump_type=prerelease 只能用于 alpha/beta 通道")
    if bump == "patch":
        new = fmt(major, minor, patch, None) if pre else fmt(major, minor, patch + 1, None)
    elif bump == "minor":
        new = fmt(major, minor + 1, 0, None)
    else:
        new = fmt(major + 1, 0, 0, None)
else:
    if bump == "prerelease":
        if pre and pre.startswith(channel + "."):
            parts = pre.split(".")
            parts[-1] = str(int(parts[-1]) + 1) if parts[-1].isdigit() else "0"
            new = fmt(major, minor, patch, ".".join(parts))
        elif pre:
            new = fmt(major, minor, patch, channel + ".0")
        else:
            new = fmt(major, minor, patch + 1, channel + ".0")
    else:
        if bump == "patch":
            base = (major, minor, patch + 1)
        elif bump == "minor":
            base = (major, minor + 1, 0)
        else:
            base = (major + 1, 0, 0)
        new = fmt(base[0], base[1], base[2], channel + ".0")

toml.write_text(
    re.sub(r'^version = "[^"]+"', 'version = "' + new + '"', text, count=1, flags=re.M)
)

lock = Path("Cargo.lock")
if lock.exists():
    lock_text = lock.read_text()
    pattern = re.compile(
        r'(\[\[package\]\]'
        + chr(10)
        + r'name = "server"'
        + chr(10)
        + r'version = ")[^"]+(")',
        re.M,
    )
    if pattern.search(lock_text):
        lock.write_text(pattern.sub(r'\g<1>' + new + r'\g<2>', lock_text, count=1))

print("Bumped " + current + " -> " + new + " (channel: " + channel + ")")

env_file = os.environ.get("GITHUB_ENV")
if env_file:
    with open(env_file, "a", encoding="utf-8") as handle:
        print("VERSION=" + new, file=handle)

out_file = os.environ.get("GITHUB_OUTPUT")
if out_file:
    with open(out_file, "a", encoding="utf-8") as handle:
        print("version=" + new, file=handle)
