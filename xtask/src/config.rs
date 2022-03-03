use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, Debug)]
pub struct Config {
  pub mod_name: String,
  pub install_location: PathBuf,
}

impl Config {
  pub fn load() -> Result<Self, eyre::Report> {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let file = Path::new(&crate_dir).parent().unwrap().join("./config.ron");

    let file = std::fs::read_to_string(file)?;

    Ok(ron::from_str(&file)?)
  }
}
