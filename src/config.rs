use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;

/// 默认监听端口
pub const DEFAULT_PORT: u16 = 4819;

const DEFAULT_DB_PATH: &str = "./mind_pulse.sqlite";

/// 服务启动配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// 监听地址
    pub bind_addr: SocketAddrV4,
    /// SQLite 数据库路径
    pub db_path: PathBuf,
}

impl ServerConfig {
    /// 从命令行参数（端口）与环境变量（数据库路径）读取配置
    pub fn from_env_and_args() -> Self {
        // 解析命令行参数获取端口号，默认为 4819
        let port = env::args()
            .nth(1)
            .and_then(|arg| arg.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);

        let db_path = env::var("MIND_PULSE_DB_PATH").unwrap_or_else(|_| DEFAULT_DB_PATH.to_owned());

        Self {
            bind_addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, port),
            db_path: PathBuf::from(db_path),
        }
    }
}
