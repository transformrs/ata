use crate::Task;
use std::fs::File;
use std::io::Write;
use transformrs::Provider;

#[derive(clap::Parser)]
pub(crate) struct TextToSpeechArgs {
    /// Voice to use for text-to-speech (optional)
    #[arg(long)]
    voice: Option<String>,

    /// Model to use (optional)
    #[arg(long)]
    model: Option<String>,

    /// Output file (optional)
    #[arg(long, short = 'o')]
    output: Option<String>,

    /// Language code (optional)
    #[arg(long)]
    language_code: Option<String>,

    /// Output format (optional)
    #[arg(long)]
    output_format: Option<String>,
}

fn default_output_format(provider: &Provider) -> Option<String> {
    match provider {
        Provider::DeepInfra => Some("mp3".to_string()),
        _ => None,
    }
}

fn default_voice(provider: &Provider) -> Option<String> {
    match provider {
        Provider::OpenAI => Some("alloy".to_string()),
        Provider::Google => Some("en-US-Studio-Q".to_string()),
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

fn default_language_code(provider: &Provider) -> Option<String> {
    match provider {
        Provider::Google => Some("en-US".to_string()),
        _ => None,
    }
}

pub(crate) async fn tts(args: &TextToSpeechArgs, key: &transformrs::Key, input: &str) {
    let provider = key.provider.clone();
    let config = transformrs::text_to_speech::TTSConfig {
        voice: args.voice.clone().or_else(|| default_voice(&provider)),
        output_format: args
            .output_format
            .clone()
            .or_else(|| default_output_format(&provider)),
        language_code: args
            .language_code
            .clone()
            .or_else(|| default_language_code(&provider)),
        ..Default::default()
    };
    let model = args
        .model
        .clone()
        .or_else(|| default_model(&provider, &Task::TTS));
    eprintln!(
        "Requesting text to speech for text of length {}...",
        input.len()
    );
    let resp = transformrs::text_to_speech::tts(key, &config, model.as_deref(), input)
        .await
        .unwrap()
        .structured()
        .unwrap();
    let bytes = resp.audio.clone();
    eprintln!("Received audio.");
    if let Some(output) = args.output.clone() {
        let mut file = File::create(output).unwrap();
        file.write_all(&bytes).unwrap();
    } else {
        std::io::stdout().write_all(&bytes).unwrap();
    }
}
