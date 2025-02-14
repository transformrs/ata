use futures_util::stream::StreamExt;
use std::fs::File;
use std::io::Write;
use transformrs::Message;
use transformrs::Provider;

#[derive(clap::Parser)]
pub(crate) struct ChatArgs {
    /// Model to use (optional)
    #[arg(long)]
    model: Option<String>,

    /// Output file (optional)
    #[arg(long, short = 'o')]
    output: Option<String>,

    /// Stream output
    #[arg(long, default_value_t = true)]
    stream: bool,

    /// Raw JSON output
    #[arg(long)]
    raw_json: bool,

    /// Language code (optional)
    #[arg(long)]
    language_code: Option<String>,
}

fn default_model(provider: &Provider) -> String {
    match provider {
        Provider::Google => "models/gemini-1.5-flash",
        Provider::OpenAI => "gpt-4o-mini",
        _ => "meta-llama/Llama-3.3-70B-Instruct",
    }
    .to_string()
}

pub(crate) async fn chat(args: &ChatArgs, key: &transformrs::Key, input: &str) {
    let provider = key.provider.clone();
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| default_model(&provider));
    let messages = vec![Message::from_str("user", input)];
    if args.stream {
        let mut stream =
            transformrs::chat::stream_chat_completion(&provider, key, &model, &messages)
                .await
                .expect("Streaming chat completion failed");
        while let Some(resp) = stream.next().await {
            let msg = resp.choices[0].delta.content.clone().unwrap_or_default();
            print!("{}", msg);
            // Ensure the output is printed immediately.
            std::io::stdout().flush().unwrap();
        }
    } else {
        let resp = transformrs::chat::chat_completion(&provider, key, &model, &messages)
            .await
            .expect("Chat completion failed");
        if args.raw_json {
            let json = resp.raw_value();
            println!("{}", json.unwrap());
        }
        let resp = resp.structured().expect("Could not parse response");
        let content = resp.choices[0].message.content.clone();
        if let Some(output) = args.output.clone() {
            let mut file = File::create(output).unwrap();
            file.write_all(content.to_string().as_bytes()).unwrap();
        } else {
            println!("{}", content);
        }
    }
}
