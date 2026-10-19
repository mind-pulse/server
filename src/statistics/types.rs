use serde::Serialize;

use crate::error::Error;

/// 客户端类型
#[derive(sqlx::Type, Default, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ClientType {
    Wechat = 1,
    #[default]
    MobileBrowser = 2,
}

impl TryFrom<u8> for ClientType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClientType::Wechat),
            2 => Ok(ClientType::MobileBrowser),
            invalid => {
                error!(message = "Invalid client type", value = invalid);
                Err(Error::InvalidClientType(invalid))
            }
        }
    }
}

/// 量表统计结果
#[derive(Debug, Serialize)]
pub struct ScaleStatistics<'a> {
    pub name: &'a str,
    pub count: u64,
}
