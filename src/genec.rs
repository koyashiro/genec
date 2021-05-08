use std::collections::HashMap;

use crate::editorconfig::Config;

#[allow(dead_code)]
pub type ConfigSet = HashMap<String, Config>;

pub mod config {
    use std::{fs::File, path::PathBuf};

    use anyhow::{Context, Result};
    use dirs::config_dir;

    #[allow(dead_code)]
    pub const CONFIG_ENV: &str = "GENEC_CONFIG";

    #[allow(dead_code)]
    pub struct Config {
        base_config_path: String,
    }

    #[allow(dead_code)]
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = config_dir().context("failed to get config dir")?;
        let config_path = config_dir.join("genec").join("config");
        Ok(config_path)
    }

    #[allow(dead_code)]
    pub fn config_file() -> Result<File> {
        let config_path = config_path()?;
        let file = File::create(config_path)?;
        Ok(file)
    }

    #[cfg(test)]
    mod config_test {
        use dirs::home_dir;

        use super::*;

        #[test]
        fn config_path_test() {
            let config_path = config_path().unwrap();
            let home_path = home_dir().expect("failed to get home dir");
            assert_eq!(
                config_path,
                home_path.join(".config").join("genec").join("config")
            );
        }
    }
}
