mod error;
mod logger;
mod scale;
mod statistics;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tracing;

use salvo::prelude::*;
use salvo::writing::Json;
use time::macros::{format_description, offset};
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::error::MindPulseResult;
use crate::logger::Logger;
use crate::scale::{
    BECK_DEPRESSION_INVENTORY, ENNEAGRAM_PERSONALITY_TEST,
    EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE, HAMILTON_DEPRESSION_SCALE,
    HOLLAND_OCCUPATIONAL_INTEREST, NEO_PERSONALITY_INVENTORY_REVISED, PATHS,
    SELF_RATING_ANXIETY_SCALE, SELF_RATING_DEPRESSION_SCALE, SIXTEEN_PERSONALITY_FACTORS,
    SYMPTOM_CHECKLIST_90, YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE,
};
use crate::statistics::{create_table, get_statistics, insert_statistics_ip};

trait JsonRender {
    fn json<S>(&mut self, data: S)
    where
        S: serde::Serialize + std::marker::Send;
}

impl JsonRender for Response {
    fn json<S>(&mut self, data: S)
    where
        S: serde::Serialize + std::marker::Send,
    {
        self.render(Json(data))
    }
}

#[handler]
async fn h_sds(res: &mut Response) {
    res.json(HOLLAND_OCCUPATIONAL_INTEREST);
}

#[handler]
async fn neo_pi_r(res: &mut Response) {
    res.json(NEO_PERSONALITY_INVENTORY_REVISED);
}

#[handler]
async fn sixteen_pf(res: &mut Response) {
    res.json(SIXTEEN_PERSONALITY_FACTORS);
}

#[handler]
async fn ept(res: &mut Response) {
    res.json(ENNEAGRAM_PERSONALITY_TEST);
}

#[handler]
async fn y_bocs(res: &mut Response) {
    res.json(YALE_BROWN_OBSESSIVE_COMPULSIVE_SCALE);
}

#[handler]
async fn epq_rsc(res: &mut Response) {
    res.json(EYSENCK_PERSONALITY_QUESTIONNAIRE_REVISED_SHORT_SCALE);
}

#[handler]
async fn bdi(res: &mut Response) {
    res.json(BECK_DEPRESSION_INVENTORY);
}

#[handler]
async fn hamd(res: &mut Response) {
    res.json(HAMILTON_DEPRESSION_SCALE);
}

#[handler]
async fn scl_90(res: &mut Response) {
    res.json(SYMPTOM_CHECKLIST_90);
}

#[handler]
async fn sas(res: &mut Response) {
    res.json(SELF_RATING_ANXIETY_SCALE);
}

#[handler]
async fn sds(res: &mut Response) {
    res.json(SELF_RATING_DEPRESSION_SCALE);
}

#[handler]
async fn list(res: &mut Response) {
    res.json(PATHS);
}

async fn serve() {
    let router = Router::new()
        .push(Router::with_path("list").get(list))
        .push(Router::with_path("scales").get(list))
        .push(Router::with_path("h_sds").get(h_sds))
        .push(Router::with_path("neo_pi_r").get(neo_pi_r))
        .push(Router::with_path("16pf").get(sixteen_pf))
        .push(Router::with_path("epq_rsc").get(epq_rsc))
        .push(Router::with_path("ept").get(ept))
        .push(Router::with_path("bdi").get(bdi))
        .push(Router::with_path("scl90").get(scl_90))
        .push(Router::with_path("hamd").get(hamd))
        .push(Router::with_path("sas").get(sas))
        .push(Router::with_path("y_bocs").get(y_bocs))
        .push(Router::with_path("sds").get(sds))
        .push(Router::with_path("statistics").get(insert_statistics_ip))
        .push(Router::with_path("get_statistics").get(get_statistics));

    let service = Service::new(router).hoop(Logger);

    let listener = TcpListener::new("127.0.0.1:9999").bind().await;
    Server::new(listener).serve(service).await;
}

#[tokio::main]
async fn main() -> MindPulseResult<()> {
    let timer = OffsetTime::new(
        offset!(+8),
        if cfg!(debug_assertions) {
            format_description!("[hour]:[minute]:[second].[subsecond digits:3]")
        } else {
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            )
        },
    );

    // NOTE: _guard must be a top-level variable
    let (writer, _guard) = {
        let file_appender = tracing_appender::rolling::daily("./log", "confidant-server.log");
        tracing_appender::non_blocking(file_appender)
    };

    #[cfg(debug_assertions)]
    let writer = {
        use tracing_subscriber::fmt::writer::MakeWriterExt;
        std::io::stderr.and(writer)
    };

    let log_level = if cfg!(debug_assertions) {
        Level::TRACE
    } else {
        Level::WARN
    };

    let builder = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_env_filter("server")
        .with_timer(timer)
        .with_writer(writer);

    #[cfg(debug_assertions)]
    builder.init();

    #[cfg(not(debug_assertions))]
    builder.json().init();

    create_table().await?;

    serve().await;

    Ok(())
}
