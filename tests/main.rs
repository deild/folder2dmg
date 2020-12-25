// Used for writing assertions
use std::process::Command;
use std::env;

use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;

// Run programs

//Given_Preconditions_When_StateUnderTest_Then_ExpectedBehavior

#[test]
fn when_run_with_option_help_then_display_usage_with_success() -> Result<(), Box<dyn std::error::Error>> {
    //Given
    let expected = "foldertodmg 0.1.2
Samuel Marcaille
foldertodmg is a personnal helper to create image from directory

USAGE:
    foldertodmg [FLAGS] [OPTIONS] <srcfolder>

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
    let mut cmd = Command::cargo_bin("foldertodmg")?;
    cmd.arg("--help");
    //Then
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));
    Ok(())
}

#[test]
fn when_run_without_options_then_display_usage_and_failure() -> Result<(), Box<dyn std::error::Error>> {
    //Given
    let expected = "error: The following required arguments were not provided:
    <srcfolder>

USAGE:
    foldertodmg [FLAGS] [OPTIONS] <srcfolder>

For more information try --help
";
    //When
    let mut cmd = Command::cargo_bin("foldertodmg")?;
    //Then
    cmd.assert()
        .failure()
        .stderr(predicate::str::similar(expected));
    Ok(())
}

#[test]
fn given_srcfolder_when_run_with_erase_then_success() -> Result<(), Box<dyn std::error::Error>> {
    //Given
    let current_path = env::current_dir()?;
    let src = "tests/folders/erase";
    let expected = format!("created: {}/tests/output/image.dmg
", current_path.display());
    Command::new("mkdir")
        .args(&["-p", src])
        .output()
        .expect("failed to execute mkdir");
    //When
    let mut cmd = Command::cargo_bin("foldertodmg")?;
    cmd.arg("--erase")
        .arg("-i").arg("tests/output/image.dmg")
        .arg(src);

    //Then
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));
    Command::new("rm")
        .args(&["test/image.dmg"])
        .output()
        .expect("failed to execute rm");
    Ok(())
}

#[test]
fn given_folder_when_dont_exist_then_failure() -> Result<(), Box<dyn std::error::Error>> {
    //Given
    let src = "tests/folders/notexist";
    let expected = "thread 'main' panicked at 'tests/folders/notexist doesn't exist', src/main.rs:54:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
";
    //When
    let mut cmd = Command::cargo_bin("foldertodmg")?;
    cmd.arg(src);
    //Then
    cmd.assert()
        .failure()
        .stderr(predicate::str::similar(expected));
    Ok(())
}