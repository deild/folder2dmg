use std::env;
// Used for writing assertions
use std::process::Command;

use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;

static PROGRAM: &str = "folder2dmg";
// Run programs

//Given_Preconditions_When_StateUnderTest_Then_ExpectedBehavior

#[test]
fn when_run_with_option_help_then_display_usage_with_success(
) -> Result<(), Box<dyn std::error::Error>> {
  //Given
  let expected = "folder2dmg 0.1.5-alpha.0
Samuel Marcaille
folder2dmg is a personnal helper to create image from directory

USAGE:
    folder2dmg [FLAGS] [OPTIONS] <srcfolder>

FLAGS:
    -d, --debug      turn on debugging information
    -e, --erase      delete the srcfolder after created the image
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --image <image>        the image to be created. Use the folder name and local Location if not set
    -v, --volname <volname>    the newly-created filesystem will be named volname. Use the folder name if not set

ARGS:
    <srcfolder>    the source directory, copies file-by-file the contents of source into
                                              image
";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  cmd.arg("--help");
  //Then
  cmd
    .assert()
    .success()
    .stdout(predicate::str::similar(expected));
  Ok(())
}

#[test]
fn when_run_without_options_then_display_usage_and_failure(
) -> Result<(), Box<dyn std::error::Error>> {
  //Given
  let expected = "error: The following required arguments were not provided:
    <srcfolder>

USAGE:
    folder2dmg [FLAGS] [OPTIONS] <srcfolder>

For more information try --help
";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  //Then
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::similar(expected));
  Ok(())
}

#[test]
fn given_srcfolder_and_image_when_erase_then_success() -> Result<(), Box<dyn std::error::Error>> {
  //Given
  let current_path = env::current_dir()?;
  let src = "tests/folders/erase";
  let image = "tests/output/image.dmg";
  let expected = format!(
    "created: {}/{}
removed {}
",
    current_path.display(),
    image,
    src
  );
  Command::new("mkdir")
    .args(&["-p", src])
    .output()
    .expect("failed to execute mkdir");
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  cmd.arg("--erase").arg("-i").arg(image).arg(src);

  //Then
  cmd
    .assert()
    .success()
    .stdout(predicate::str::similar(expected));
  Ok(())
}

#[test]
fn given_folder_when_dont_exist_then_failure() -> Result<(), Box<dyn std::error::Error>> {
  //Given
  let src = "tests/folders/notexist";
  let expected = "error: 'tests/folders/notexist' directory doesn't exist
";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  cmd.arg(src);
  //Then
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::similar(expected));
  Ok(())
}
