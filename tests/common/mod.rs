use assert_cmd::Command;
use std::io::BufRead;
use transformrs::Provider;

pub fn trf() -> Command {
    Command::cargo_bin("trf").unwrap()
}

#[allow(dead_code)]
/// Load a key from the local .env file.
///
/// This is used for testing only. Expects the .env file to contain keys for providers in the following format:
///
/// ```
/// DEEPINFRA_KEY="<KEY>"
/// OPENAI_KEY="<KEY>"
/// ```
pub fn load_key(provider: &Provider) -> String {
    fn finder(line: &Result<String, std::io::Error>, provider: &Provider) -> bool {
        line.as_ref().unwrap().starts_with(&provider.key_name())
    }
    let path = std::path::Path::new("test.env");
    let file = std::fs::File::open(path).expect("Failed to open .env file");
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();
    let key = lines.find(|line| finder(line, provider)).unwrap().unwrap();
    key.split("=").nth(1).unwrap().to_string()
}
