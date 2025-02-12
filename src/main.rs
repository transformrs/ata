use clap::Parser;
use std::io::Read;
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
}

enum Task {
    Chat,
    TTS,
}

fn find_single_key(keys: transformrs::Keys) -> Key {
    // let n = keys.keys.len();
    let keys = keys.keys;
    if keys.len() != 1 {
        eprintln!("Expected exactly one key, got {}", keys.len());
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
            Task::Chat => Some("gpt-4o".to_string()),
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

    // TODO: add logic to transformrs to just get the key without .env file.
    let keys = transformrs::load_keys(".env");
    let key = find_single_key(keys);
    let provider = key.provider.clone();

    if args.tts {
        let mut config = transformrs::text_to_speech::TTSConfig::default();
        config.voice = args.voice.or_else(|| default_voice(&provider));
        config.output_format = Some("mp3".to_string());
        let model = args.model.or_else(|| default_model(&provider, &Task::TTS));
        let resp = transformrs::text_to_speech::tts(&key, &config, model.as_deref(), &input)
            .await
            .unwrap()
            .structured()
            .unwrap();
    } else {
        error_and_exit("No action specified.");
    }
}
