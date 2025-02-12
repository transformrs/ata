use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about = "Ask the Terminal Anything - Use AI in the terminal")]
struct Arguments {
    /// Enable text-to-speech conversion
    #[arg(long)]
    tts: bool,

    /// Voice to use for text-to-speech (optional)
    #[arg(long)]
    voice: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // TODO: add logic to transformrs to just get the key without .env file.
    let _openai_key = std::env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");

    if args.tts {
        if let Some(voice) = args.voice {
            todo!("Implement TTS with voice: {}", voice);
        } else {
            todo!("Implement TTS with default voice");
        }
    } else {
        eprintln!("No action specified. Use --tts to convert text to speech.");
        std::process::exit(1);
    }
}
