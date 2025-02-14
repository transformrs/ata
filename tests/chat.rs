mod common;

use common::load_key;
use common::trf;
use predicates::prelude::*;
use transformrs::Provider;

fn canonicalize_response(text: &str) -> String {
    text.to_lowercase()
        .trim()
        .trim_end_matches('.')
        .trim_end_matches('!')
        .to_string()
}

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
fn tts_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = trf();
    let key = load_key(&Provider::DeepInfra);
    let cmd = cmd
        .arg("chat")
        .env("DEEPINFRA_KEY", key)
        .write_stdin("This is a test. Respond with 'hello'.")
        .current_dir(&dir);
    let output = cmd.assert().success().get_output().stdout.clone();

    let text = String::from_utf8(output.clone()).unwrap();
    let content = canonicalize_response(&text);
    assert_eq!(content, "hello");
    Ok(())
}
