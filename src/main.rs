use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use clap::{App, Arg, ArgMatches, crate_authors, crate_description, crate_name, crate_version};

fn run(matches: ArgMatches) {
    // hdiutil create -volname "$dest" -srcfolder "./$dirsrc" -ov -format UDBZ -nospotlight "${dest}.dmg"
    let (src_folder, image, volname, debug, erase) = init(&matches);
    let args = build_args(src_folder, image.as_str(), volname, debug);

    let output = Command::new("hdiutil")
        .args(args)
        .output()
        .expect("failed to execute hdiutil");
    if debug {
        println!("status: {}", output.status);
    }
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if erase {
        let output_rm= Command::new("rm")
            .args(&["-vf", src_folder])
            .output()
            .expect("failed to execute rm");
        io::stdout().write_all(&output_rm.stdout).unwrap();
        io::stderr().write_all(&output_rm.stderr).unwrap();
    }
    assert!(output.status.success());
    // hdiutil create -volname "$dest" -srcfolder "./$dirsrc" -ov -format UDBZ -nospotlight "${dest}.dmg"
}

fn init<'a>(matches: &'a ArgMatches) -> (&'a str, String, &'a str, bool, bool) {
    let path = Path::new(matches.value_of("srcfolder").unwrap());
    chech_path(path);
    let src_folder = path.to_str().unwrap();
    let folder_name = src_folder.split("/").last().unwrap();
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
    (src_folder, image, volname, matches.is_present("debug"), matches.is_present("erase"))
}

fn chech_path(path: &Path) {
    if !path.exists() {
        panic!("{} doesn't exist", path.to_str().unwrap());
    }
    if !path.is_dir() {
        panic!("{} is not a directory", path.to_str().unwrap());
    }
}

fn build_args<'a>(src_folder: &'a str, image: &'a str, volname: &'a str, debug: bool) -> Vec<&'a str> {
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

fn options() -> ArgMatches<'static> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("image")
                .help("the image to be created. Use the folder name and local Location if not set")
                .takes_value(true)
                .short("i")
                .long("image")
        )
        .arg(
            Arg::with_name("srcfolder")
                .help("the source directory, copies file-by-file the contents of source into
                           image")
                .required(true)
        )
        .arg(
            Arg::with_name("volname")
                .help("the newly-created filesystem will be named volname. Use the folder name if not set")
                .takes_value(true)
                .short("v")
                .long("volname")
        )
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short("d")
                .long("debug"),
        )
        .arg(
            Arg::with_name("erase")
                .help("delete the srcfolder after created the image")
                .short("e")
                .long("erase"),
        )
        .get_matches();
    matches
}

fn main() {
    let matches = options();
    run(matches)
}
