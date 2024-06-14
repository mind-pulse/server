use std::time::SystemTimeError;

use salvo::{async_trait, http::StatusCode, writing::Text, Depot, Request, Response, Writer};

pub type ConfidantResult<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
    #[error(transparent)]
    Sqlite(#[from] sqlx::Error),
    #[error(transparent)]
    SystemTime(#[from] SystemTimeError),
    #[error("无效的客户端类型：{0}")]
    InvalidClientType(u8),
    #[error("{0}")]
    Response(String),
}

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match &self {
            Error::Response(_) => res.status_code(StatusCode::BAD_REQUEST),
            _ => res.status_code(StatusCode::INTERNAL_SERVER_ERROR),
        };

        res.render(Text::Plain(self.to_string()));
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Response(value.to_string())
    }
}
