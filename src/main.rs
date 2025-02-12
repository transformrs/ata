mod tts;

use clap::Parser;
use std::io::Read;
use transformrs::Key;
use tts::TextToSpeechArgs;

#[derive(clap::Subcommand)]
enum Commands {
    /// Convert text to speech
    ///
    /// Takes text input from stdin and converts it to speech using text-to-speech models.
    #[command()]
    Tts(TextToSpeechArgs),
}

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Ask the Terminal Anything - Use AI in the terminal"
)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

pub enum Task {
    #[allow(clippy::upper_case_acronyms)]
    TTS,
}

fn find_single_key(keys: transformrs::Keys) -> Key {
    let keys = keys.keys;
    if keys.len() != 1 {
        eprintln!("Expected exactly one key, found {}", keys.len());
        std::process::exit(1);
    }
    keys[0].clone()
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let keys = transformrs::load_keys(".env");
    let key = find_single_key(keys);

    match args.command {
        Commands::Tts(args) => {
            tts::tts(&args, &key, &input).await;
        }
    }
}
