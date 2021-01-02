use clap::{
  crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};

pub fn define_options() -> ArgMatches<'static> {
  let image_arg = Arg::with_name("image")
    .help("The image to be created. Use the folder name and local Location if not set")
    .takes_value(true)
    .short("i")
    .long("image");

  let srcfolder_arg = Arg::with_name("srcfolder")
    .help(
      "the source directory, copies file-by-file the contents of source into
                           image",
    )
    .required(true);

  let volname_arg = Arg::with_name("volname")
    .help("The newly-created filesystem will be named volname. Use the folder name if not set")
    .takes_value(true)
    .short("v")
    .long("volname");

  let erase_arg = Arg::with_name("erase")
    .help("Delete the srcfolder after created the image")
    .short("e")
    .long("erase");

  let debug_arg = Arg::with_name("debug")
    .help("Turn on debugging information")
    .short("d")
    .long("debug");

  let quiet_arg = Arg::with_name("quiet")
    .help("No output printed to stdout")
    .short("q")
    .long("quiet");

  let matches = App::new(crate_name!())
    .about(crate_description!())
    .version(crate_version!())
    .author(crate_authors!())
    .after_help("https://github.com/deild/folder2dmg")
    .subcommand(
      SubCommand::with_name("bug-report")
        .about("Create a pre-populated GitHub issue with information about your configuration"),
    )
    .subcommand(
      SubCommand::with_name("build")
        .about("Create an image from srcfolder")
        .args(&[
          image_arg,
          srcfolder_arg,
          volname_arg,
          debug_arg,
          erase_arg,
          quiet_arg,
        ]),
    )
    .get_matches();
  matches
}
