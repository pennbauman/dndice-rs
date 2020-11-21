// integration tests - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use assert_cmd::prelude::*;
use std::process::Command;


// Check split string and vector match, numbers are replaced with "" in vec
fn check_pattern(output: String, pattern: Vec<&str>) {
    let mut i = 0;
    for w in output.split_whitespace() {
        if pattern[i] == "" {
            assert!(w.parse::<i32>().is_ok());
        } else {
            assert_eq!(pattern[i], w);
        }
        i += 1;
    }
}


// General errors
#[test]
fn test_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.assert().failure().stderr(
        predicates::str::contains("No dice or command provided"));
    Ok(())
}
#[test]
fn test_invalid_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("--yeet");
    cmd.assert().failure().stderr(
        predicates::str::contains("Invalid option"));
    cmd = Command::cargo_bin("dndice")?;
    cmd.arg("-y");
    cmd.assert().failure().stderr(
        predicates::str::contains("Invalid option"));
    Ok(())
}

// Standalone options
#[test]
fn test_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("--version");
    cmd.assert().success().stdout(
        predicates::str::contains(env!("CARGO_PKG_VERSION")));
    Ok(())
}
#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("--help").arg("foo").arg("bar");
    cmd.assert().success().stdout(
        predicates::str::contains("Usage: dndice [command] [dice] [options]"));

    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("-1234").arg("-h");
    cmd.assert().success().stdout(
        predicates::str::contains("Usage: dndice [command] [dice] [options]"));
    Ok(())
}

// Stats
#[test]
fn test_stats_no_method() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats");
    cmd.assert().failure().stderr(
        predicates::str::contains("No statistics generation method provided"));
    Ok(())
}
#[test]
fn test_stats_bad_method() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("go");
    cmd.assert().failure().stderr(
        predicates::str::contains("Unknown statistics generation method"));
    Ok(())
}
#[test]
fn test_stats_too_many_methods() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("go").arg("go");
    cmd.assert().failure().stderr(
        predicates::str::contains("Too many statistics generation methods provided"));
    Ok(())
}
#[test]
fn test_stats_std() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("std");
    cmd.assert().success().stdout(
        predicates::str::contains("Stats:\n15 14 13 12 10  8"));
    Ok(())
}
#[test]
fn test_stats_d20() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("d20");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["Stats:", "", "", "", "", "", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_stats_4d6() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("4d6");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["Stats:", "", "", "", "", "", ""];
    check_pattern(output, expected);
    Ok(())
}

// Dice
#[test]
fn test_dice_wide_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("1d8").arg("+").arg("3");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["1d8", "+", "3", "|", "", "Result:", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_dice_no_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("4d6+1d12");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["4d6", "+", "1d12", "|", "d6:", "", "", "", "", "|",
        "d12:", "", "Result:", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_dice_risky_spacing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("3d7").arg("-").arg("4").arg("-1d10");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["3d7", "-", "4", "-", "1d10", "|", "d7:", "", "", "", "|",
        "d10:", "", "Result:", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_dice_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("roll").arg("1d4").arg("-1");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["1d4", "-", "1", "|", "", "Result:", ""];
    check_pattern(output, expected);
    Ok(())
}

// --number N
#[test]
fn test_number_option_stats() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("d20").arg("--number").arg("4");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["Stats:", "", "", "", "", "", "",
        "", "", "", "", "", "",
        "", "", "", "", "", "",
        "", "", "", "", "", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_number_option_dice() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("1d8").arg("+").arg("3").arg("-n").arg("2");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["1d8", "+", "3", "|", "", "Result:", "",
        "1d8", "+", "3", "|", "", "Result:", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_number_option_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("-n").arg("X");
    cmd.assert().failure().stderr(
        predicates::str::contains("Invalid number"));
    Ok(())
}

// --quiet
#[test]
fn test_quiet_option_stats() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("stats").arg("d20").arg("--quiet");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec!["", "", "", "", "", ""];
    check_pattern(output, expected);
    Ok(())
}
#[test]
fn test_quiet_option_dice() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dndice")?;
    cmd.arg("1d8").arg("+").arg("3").arg("-q");
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let expected = vec![""];
    check_pattern(output, expected);
    Ok(())
}
