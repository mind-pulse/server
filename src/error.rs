use salvo::{Depot, Request, Response, Writer, async_trait, http::StatusCode, writing::Text};

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// 服务统一错误类型（领域错误 + 基础设施错误）
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    #[error("无效的量表 ID")]
    ScaleNotFound,
    #[error("无效的客户端类型：{0}")]
    InvalidClientType(u8),
    #[error("服务内部状态缺失")]
    StateMissing,
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ScaleNotFound => StatusCode::NOT_FOUND,
            Error::InvalidClientType(_) => StatusCode::BAD_REQUEST,
            Error::StateMissing | Error::Database(_) | Error::Serialization(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(self.status_code());
        res.render(Text::Plain(self.to_string()));
    }
}
