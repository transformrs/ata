mod common;

use common::load_key;
use common::trf;
use predicates::prelude::*;
use transformrs::Provider;

#[test]
fn unexpected_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = trf();
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));

    Ok(())
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = trf();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: trf"));

    Ok(())
}

#[test]
fn tts_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = trf();
    let key = load_key(&Provider::DeepInfra);
    let cmd = cmd
        .arg("tts")
        .env("DEEPINFRA_KEY", key)
        .write_stdin("Hello world")
        .current_dir(&dir);
    let output = cmd.assert().success().get_output().stdout.clone();

    assert!(output.len() > 0);

    Ok(())
}

fn tts_default_settings_helper(provider: &Provider) -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = trf();
    let key = load_key(provider);
    let name = provider.key_name();
    cmd.arg("tts")
        .arg("--output")
        .arg("output.mp3")
        .env(name, key)
        .write_stdin("Hi")
        .current_dir(&dir)
        .assert()
        .success();

    let path = dir.path().join("output.mp3");
    assert!(path.exists());

    Ok(())
}

#[test]
fn tts_no_args_deepinfra() -> Result<(), Box<dyn std::error::Error>> {
    tts_default_settings_helper(&Provider::DeepInfra)
}

#[test]
fn tts_no_args_google() -> Result<(), Box<dyn std::error::Error>> {
    tts_default_settings_helper(&Provider::Google)
}

#[test]
fn tts_no_args_openai() -> Result<(), Box<dyn std::error::Error>> {
    tts_default_settings_helper(&Provider::OpenAI)
}
