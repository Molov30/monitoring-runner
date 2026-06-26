use std::{path::PathBuf, process::Command};

pub struct Config {
    pub monitoring_bin_path: PathBuf,
    pub monitoring_port: u64,
    pub monitoring_host: String,
    pub monigoring_db_dir: PathBuf,
    pub nbl_grpc_port: u64,
    pub prometheus_port: u64,
    pub gprc_token: String,
}

impl Config {
    fn env_vars(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "LOCALMONITORING_PROMETHEUS_URL",
                format!("http://127.0.0.1:{}", self.prometheus_port),
            ),
            (
                "LOCALMONITORING_VMS_ENDPOINT",
                format!("127.0.0.1:{}", self.nbl_grpc_port),
            ),
            ("LOCALMONITORING_NBL_TOKEN", self.gprc_token.clone()),
            (
                "NGP_PLUGINS_DATA_PATH",
                self.monigoring_db_dir.to_string_lossy().into_owned(),
            ),
            ("LOCALMONITORING_PORT", self.monitoring_port.to_string()),
            ("HOSTNAME", self.monitoring_host.clone()),
        ]
    }
}

pub fn launch(cfg: &Config) -> Result<(), String> {
    let status = Command::new(&cfg.monitoring_bin_path)
        .envs(cfg.env_vars())
        .status()
        .map_err(|e| format!("failed to run local monitoring: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("proccess exited with {}", status))
    }
}
