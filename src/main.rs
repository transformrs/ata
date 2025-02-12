use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use transformrs::Key;
use transformrs::Provider;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Ask the Terminal Anything - Use AI in the terminal"
)]
struct Arguments {
    /// Enable text-to-speech conversion
    #[arg(long)]
    tts: bool,

    /// Voice to use for text-to-speech (optional)
    #[arg(long)]
    voice: Option<String>,

    /// Model to use (optional)
    #[arg(long)]
    model: Option<String>,

    /// Output file (optional)
    #[arg(long, short = 'o')]
    output: Option<String>,
}

enum Task {
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

fn error_and_exit(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn default_voice(provider: &Provider) -> Option<String> {
    match provider {
        Provider::OpenAI => Some("alloy".to_string()),
        _ => None,
    }
}

fn default_model(provider: &Provider, task: &Task) -> Option<String> {
    match provider {
        Provider::OpenAI => match task {
            Task::TTS => Some("tts-1".to_string()),
        },
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let keys = transformrs::load_keys(".env");
    let key = find_single_key(keys);
    let provider = key.provider.clone();

    if args.tts {
        let mut config = transformrs::text_to_speech::TTSConfig::default();
        config.voice = args.voice.or_else(|| default_voice(&provider));
        config.output_format = Some("mp3".to_string());
        let model = args.model.or_else(|| default_model(&provider, &Task::TTS));
        eprintln!("Requesting text to speech for text of length {}...", input.len());
        let resp = transformrs::text_to_speech::tts(&key, &config, model.as_deref(), &input)
            .await
            .unwrap()
            .structured()
            .unwrap();
        let bytes = resp.audio.clone();
        eprintln!("Received audio.");
        if let Some(output) = args.output {
            let mut file = File::create(output).unwrap();
            file.write_all(&bytes).unwrap();
        } else {
            std::io::stdout().write_all(&bytes).unwrap();
        }
    } else {
        error_and_exit("No action specified.");
    }
}
