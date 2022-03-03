use std::path::Path;

mod config;
use config::Config;

const HELP: &str = "\
App
USAGE:
  app [OPTIONS]
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  build                 Compiles the DLL
  run                   Compiles and runs Civ4
";

fn main() -> Result<(), eyre::Report> {
  color_eyre::install()?;
  let mut args = std::env::args().skip(1);

  let first = args.next();
  let first = first.as_deref();

  let config = Config::load()?;

  match first {
    Some("build") => build(&config, args),
    Some("run") => run(&config, args),
    _ => {
      println!("{}", HELP);
      Ok(())
    },
  }
}

fn print(verb: &'static str, cmd: &'static str) {
  use owo_colors::OwoColorize;

  println!("{} {}", verb.bold().green(), format!("`{}`", cmd).italic());
}

fn build(_: &Config, mut args: impl Iterator<Item = String>) -> Result<(), eyre::Report> {
  let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

  let valid_profiles = ["release", "debug"];

  let profile = args.next().unwrap_or_else(|| String::from("debug"));

  if !valid_profiles.contains(&profile.as_str()) {
    return Err(eyre::eyre!(
      "profile must be one of the following: {}",
      valid_profiles.join(", ")
    ));
  }

  print("building", "DLL");

  std::env::set_current_dir(Path::new(&crate_dir).parent().unwrap())?;

  duct::cmd!(
    "cargo",
    "build",
    "--package",
    "dll",
    "--target",
    "i686-pc-windows-msvc"
  )
  .run()?;

  duct::cmd!(
    "bash",
    "-c",
    format!("cd ./CvGameCoreDLL && ./compile.sh {}", profile)
  )
  .run()?;

  Ok(())
}

fn run(config: &Config, args: impl Iterator<Item = String>) -> Result<(), eyre::Report> {
  let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

  build(config, args)?;

  print("copying", "DLL");

  let from = Path::new(&crate_dir)
    .parent()
    .unwrap()
    .join("CvGameCoreDLL")
    .join("Output")
    .join("CvGameCoreDLL.dll");

  let to = Path::new(&config.install_location)
    .join("Beyond the Sword")
    .join("Mods")
    .join(&config.mod_name)
    .join("Assets")
    .join("CvGameCoreDLL.dll");

  std::fs::copy(from, to)?;

  print("running", "Civ 4");

  duct::cmd!(
    "steam",
    "-applaunch",
    "8800",
    format!(r#""mod=\{}""#, config.mod_name)
  )
  .run()?;

  Ok(())
}
