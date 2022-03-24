use std::path::Path;

mod config;
use config::Config;

mod dump;

const HELP: &str = "\
App
USAGE:
  app [OPTIONS]
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  build                 Compiles the DLL
  run                   Compiles and runs Civ4
  dump [class]          Dumps a starting point for binding the given class
";

fn main() -> Result<(), eyre::Report> {
  color_eyre::install()?;
  let mut args = std::env::args().skip(1);

  let first = args.next();
  let first = first.as_deref();

  let config = Config::load()?;

  match first {
    Some("build") => build(&config, &mut args),
    Some("run") => run(&config, &mut args),
    Some("dump") => dump(&config, &mut args),
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

fn build(_: &Config, args: &mut impl Iterator<Item = String>) -> Result<(), eyre::Report> {
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

fn run(config: &Config, args: &mut impl Iterator<Item = String>) -> Result<(), eyre::Report> {
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

fn dump(config: &Config, args: &mut impl Iterator<Item = String>) -> Result<(), eyre::Report> {
  build(config, &mut vec![String::from("debug")].into_iter())?;

  let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

  let from = Path::new(&crate_dir)
    .parent()
    .unwrap()
    .join("CvGameCoreDLL")
    .join("Output")
    .join("CvGameCoreDLL.dll");

  let file_map = pelite::FileMap::open(&from).unwrap();
  // Process the image file
  let classes = dump::dump(file_map.as_ref()).unwrap();

  let class = match args.next() {
    Some(class) => class,
    None => return Err(eyre::eyre!("forgot to supply a class!",)),
  };

  let methods = match classes.get(&class) {
    Some(methods) => methods,
    None => return Err(eyre::eyre!("Class doesn't exist: {}", class)),
  };

  let methods = methods
    .iter()
    .map(|v| format!("  {}", v.parsed.clone()))
    .collect::<Vec<_>>()
    .join("");

  let dump = format!(
    r#"
extern "thiscall" {{
  pub type {class};
  {methods}
}}"#
  );

  std::fs::write(
    Path::new(&crate_dir).parent().unwrap().join("dump.rs"),
    dump,
  )?;

  Ok(())
}
