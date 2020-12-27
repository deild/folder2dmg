use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use clap::ArgMatches;

use options::define_options;

mod cli;
mod options;

fn run(matches: ArgMatches) {
  let (src_folder, image, volname, debug, erase) = init(&matches);
  let args = build_args(src_folder, image.as_str(), volname, debug);

  let output = Command::new("hdiutil")
    .args(args)
    .stdout(Stdio::inherit())
    .output()
    .expect("failed to execute hdiutil");

  if debug {
    println!("status: {}", output.status);
  }
  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
  if erase && output.status.success() {
    let output_rm = Command::new("rm")
      .args(&["-rvf", src_folder])
      .output()
      .expect("failed to execute rm");
    io::stdout().write_all(b"removed ").unwrap();
    io::stdout().write_all(&output_rm.stdout).unwrap();
    io::stderr().write_all(&output_rm.stderr).unwrap();
  }

  assert!(output.status.success());
}

fn init<'a>(matches: &'a ArgMatches) -> (&'a str, String, &'a str, bool, bool) {
  let path = Path::new(matches.value_of("srcfolder").unwrap());
  chech_path(path);
  let src_folder = path.to_str().unwrap();
  let folder_name = src_folder.split('/').last().unwrap();
  let mut image: String = String::from(folder_name);
  if matches.is_present("image") {
    image = String::from(matches.value_of("image").unwrap());
  }
  if !image.contains(".dmg") {
    image.push_str(".dmg")
  }
  let mut volname = folder_name;
  if matches.is_present("volname") {
    volname = matches.value_of("volname").unwrap();
  }
  (
    src_folder,
    image,
    volname,
    matches.is_present("debug"),
    matches.is_present("erase"),
  )
}

fn chech_path(path: &Path) {
  if !path.exists() {
    eprintln!(
      "error: '{}' directory doesn't exist",
      path.to_str().unwrap()
    );
    std::process::exit(1);
  }
  if !path.is_dir() {
    eprintln!("error: '{}' is not a directory", path.to_str().unwrap());
    std::process::exit(1);
  }
}

fn build_args<'a>(
  src_folder: &'a str,
  image: &'a str,
  volname: &'a str,
  debug: bool,
) -> Vec<&'a str> {
  let mut args = vec![];
  args.push("create");
  args.push("-volname");
  args.push(volname);
  args.push("-srcfolder");
  args.push(src_folder);
  args.push("-ov");
  args.push("-format");
  args.push("UDBZ");
  args.push("-nospotlight");
  args.push(image);
  if debug {
    args.push("-help");
    for text in &args {
      println!("[DEBUG]: {}", text);
    }
  }
  args
}

fn main() {
  let matches = define_options();
  run(matches);
}
