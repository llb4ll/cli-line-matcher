use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linematcher")?;
    cmd.arg("foobar")
        .arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn files_not_specified() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linematcher")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The following required arguments were not provided"))
        .stderr(predicate::str::contains("paths"));

    Ok(())
}
