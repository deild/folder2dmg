// Used for writing assertions
use std::process::Command;

use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;

// Run programs

#[test]
fn usage() -> Result<(), Box<dyn std::error::Error>> {
    //Given
    let expected = "foldertodmg 0.1.2
Samuel Marcaille
foldertodmg is a personnal helper to create image from directory

USAGE:
    foldertodmg [FLAGS] [OPTIONS] <srcfolder>

FLAGS:
    -d, --debug      turn on debugging information
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
    cmd.arg("-h");
    //Then
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(expected));
    Ok(())
}

#[test]
fn without_option() -> Result<(), Box<dyn std::error::Error>> {
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