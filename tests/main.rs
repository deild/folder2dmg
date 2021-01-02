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
  let expected = "folder2dmg 0.1.6-alpha.0
Samuel Marcaille
Folder2dmg is a personnal helper to create image from directory

USAGE:
    folder2dmg [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bug-report    Create a pre-populated GitHub issue with information about your configuration
    build         Create an image from srcfolder
    help          Prints this message or the help of the given subcommand(s)

https://github.com/deild/folder2dmg\n";
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
  let expected_err = "Error: one subcommand is required\n";
  let expected_out = "USAGE:
    folder2dmg [SUBCOMMAND]\n";
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  //Then
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::similar(expected_err))
    .stdout(predicate::str::similar(expected_out));
  Ok(())
}

#[test]
fn given_srcfolder_volume_and_image_when_erase_then_success(
) -> Result<(), Box<dyn std::error::Error>> {
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
  cmd.args(&[
    "build",
    "--volname",
    "NONAME",
    "--quiet",
    "--erase",
    "-i",
    image,
    src,
  ]);

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
  cmd.args(&["build", src]);
  //Then
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::similar(expected));
  Ok(())
}

#[test]
fn given_srcfolder_when_debug_then_success() -> Result<(), Box<dyn std::error::Error>> {
  //Given
  let src = "tests/folders/erase";
  let expected_stdout = "[DEBUG]: create
[DEBUG]: -volname
[DEBUG]: erase
[DEBUG]: -srcfolder
[DEBUG]: tests/folders/erase
[DEBUG]: -ov
[DEBUG]: -format
[DEBUG]: UDBZ
[DEBUG]: -nospotlight
[DEBUG]: erase.dmg
[DEBUG]: -help
status: exit code: 0
";

  Command::new("mkdir")
    .args(&["-p", src])
    .output()
    .expect("failed to execute mkdir");
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  cmd.args(&["build", "--debug", src]);

  //Then
  cmd
    .assert()
    .success()
    .stdout(predicate::str::similar(expected_stdout));
  Ok(())
}

#[test]
fn given_src_when_file_then_failure() -> Result<(), Box<dyn std::error::Error>> {
  //Given
  let src = "tests/folders/file";
  let expected = "error: Found argument 'tests/folders/file' which wasn't expected, or isn't valid in this context

USAGE:
    folder2dmg [SUBCOMMAND]

For more information try --help\n";
  Command::new("touch")
    .args(&[src])
    .output()
    .expect("failed to execute touch");
  //When
  let mut cmd = Command::cargo_bin(PROGRAM)?;
  cmd.args(&[src]);
  //Then
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::similar(expected));
  Ok(())
}
