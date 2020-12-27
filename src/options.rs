use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

pub fn define_options() -> ArgMatches<'static> {
  let matches = App::new(crate_name!())
    .about(crate_description!())
    .version(crate_version!())
    .author(crate_authors!())
    .arg(
      Arg::with_name("image")
        .help("the image to be created. Use the folder name and local Location if not set")
        .takes_value(true)
        .short("i")
        .long("image"),
    )
    .arg(
      Arg::with_name("srcfolder")
        .help(
          "the source directory, copies file-by-file the contents of source into
                           image",
        )
        .required(true),
    )
    .arg(
      Arg::with_name("volname")
        .help("the newly-created filesystem will be named volname. Use the folder name if not set")
        .takes_value(true)
        .short("v")
        .long("volname"),
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
