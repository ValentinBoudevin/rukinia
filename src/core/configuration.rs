use serde::Deserialize;
use tokio::runtime::Runtime;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use crate::core::rukinia_result::RukiniaError;

#[derive(Deserialize)]
pub struct TokioConfig {
    pub _flavor: String,
    pub _worker_threads: usize,
}

#[derive(Deserialize)]
pub struct RukiniaConfig {
    pub _tokio: TokioConfig,
}

pub fn rukinia_use_settings() -> Result<Runtime, Box<dyn Error>> {
    // let settings = config::Config::builder()
    //     .add_source(config::File::with_name("config"))
    //     .build()?;
    // let config: RukiniaConfig = settings.try_deserialize()?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    Ok(runtime)
}

pub fn rukinia_open_test_file(file: &mut Option<File>) {
    *file = match File::open("/etc/rukinia/rukinia.conf") {
        Ok(f) => Some(f),
        Err(e) => {
            RukiniaError::new(
                "rukinia init open /etc/rukinia/rukinia.conf".to_string(),
                "Failed to open /etc/rukinia/rukinia.conf".to_string(),
                e.to_string(),
            )
            .display_result();
            return;
        }
    };
}

pub fn rukinia_read_test_file(file: &mut File, buffer: &mut String) {
    if let Err(e) = file.read_to_string(buffer) {
        RukiniaError::new(
            "rukinia init read /etc/rukinia/rukinia.conf".to_string(),
            "Failed to read /etc/rukinia/rukinia.conf".to_string(),
            e.to_string(),
        )
        .display_result();
        return;
    }
}
