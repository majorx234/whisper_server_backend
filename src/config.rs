use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub host_ip: String,
    pub port: u32,
    pub frontend_dir_path: PathBuf,
    pub file_store_dir_path: PathBuf,
    pub rust_log: String,
    pub model_path: PathBuf,
    pub vad_model_path: PathBuf,
}

impl Config {
    /// Config Constructor, reads env variables and sets config
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").map_or(8080, |port_string| {
            port_string.parse::<u32>().expect("PORT not parsable")
        });
        let frontend_dir = std::env::var("FRONTEND_DIR").expect("FRONTEND_DIR not set");
        let mut frontend_dir_path = PathBuf::new();
        frontend_dir_path.push(frontend_dir);
        match frontend_dir_path.is_absolute() {
            true => (),
            false => {
                frontend_dir_path = std::env::current_dir().unwrap().join(frontend_dir_path);
            }
        };
        let file_store_dir = std::env::var("FILE_STORE_DIR").expect("FILE_STORE_DIR not set");
        // todo set folder where executed
        let rust_log =
            std::env::var("RUST_LOG").unwrap_or_else(|_| "todo_axum=debug,tower_http=debug".into());
        let model_path_var = std::env::var("MODEL_PATH").expect("MODEL_PATH not set");
        let mut model_path = PathBuf::new();
        model_path.push(model_path_var);
        let vad_model_path_var = std::env::var("VAD_MODEL_PATH").expect("VAD_MODEL_PATH not set");
        let mut vad_model_path = PathBuf::new();
        vad_model_path.push(vad_model_path_var);
        Config {
            host_ip,
            port,
            frontend_dir_path,
            file_store_dir_path: file_store_dir.into(),
            rust_log,
            model_path,
            vad_model_path,
        }
    }

    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }

    pub fn get_frontend_dir_path(&self) -> &Path {
        self.frontend_dir_path.as_path()
    }
    pub fn get_file_store_dir_path(&self) -> &Path {
        self.file_store_dir_path.as_path()
    }

    pub fn get_rust_log(&self) -> &str {
        &self.rust_log
    }

    pub fn get_model_path(&self) -> &Path {
        &self.vad_model_path.as_path()
    }

    pub fn get_vad_model_path(&self) -> &Path {
        &self.vad_model_path.as_path()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
