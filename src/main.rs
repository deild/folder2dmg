use std::io::{self, Write};
use std::process::Command;
use std::path::Path;

use clap::{App, Arg, ArgMatches, crate_authors, crate_description, crate_name, crate_version};

fn run(matches: ArgMatches) {
    // hdiutil create -volname "$dest" -srcfolder "./$dirsrc" -ov -format UDBZ -nospotlight "${dest}.dmg"
    let path = Path::new(matches.value_of("srcfolder").unwrap());
    let src_folder = path.to_str().unwrap();
    let folder_name = src_folder.split("/").last().unwrap();
    let mut image: String = String::new();
    //image.push_str("'");
    if matches.is_present("image") {
        image.push_str(matches.value_of("image").unwrap());
    } else {
        image.push_str(folder_name);
    }
    if ! image.contains(".dmg") {
        image.push_str(".dmg")
    }
    //image.push_str("'");
    let volname;
    if matches.is_present("volname") {
        volname = matches.value_of("volname").unwrap();
    } else {
        volname = folder_name;
    }
    let mut args=vec![];
    args.push("create");
    args.push("-volname");
    args.push(volname);
    args.push("-srcfolder");
    args.push(src_folder);
    args.push("-ov");
    args.push("-format");
    args.push("UDBZ");
    args.push("-nospotlight");
    args.push(image.as_str());

    if matches.is_present("debug") {
        args.push("-help");
        for text in &args {
            println!("[DEBUG]: {}", text);
        }
    }

    let output = Command::new("hdiutil")
        .args(args)
        .output()
        .expect("failed to execute hdiutil");

    println!("status: {}", output.status);
    //io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
    // hdiutil create -volname "$dest" -srcfolder "./$dirsrc" -ov -format UDBZ -nospotlight "${dest}.dmg"
}

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("image")
                .help("set the location of dmg image, use srcfolder and local Location")
                .takes_value(true)
                .short("i")
                .long("image")
        )
        .arg(
            Arg::with_name("srcfolder")
                .help("the source folder to use")
                .required(true)
        )
        .arg(
            Arg::with_name("volname")
                .help("set the Volume name, use Untitle if not set")
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
        .get_matches();
    run(matches)
}
