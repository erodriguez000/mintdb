use std::net::SocketAddr;
use once_cell::sync::OnceCell;
#[allow(unused)]
pub static CF: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: SocketAddr,
    pub user: String,
    pub pass: Option<String>
}

#[allow(unused)]
pub fn init() {
    let bind = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let user = "root".to_string();
    let pass = Some("root".to_string());
    let _ = CF.set(Config { 
        bind, 
        user, 
        pass
    });
}