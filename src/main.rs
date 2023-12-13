use std::time::Duration;
use async_openai::{
    types::{Voice, SpeechModel, CreateSpeechRequestArgs, CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, ChatCompletionRequestAssistantMessageArgs},
    Client,
};

// Usage: ask how to symlink from /etc/systemd/system/wolf.service to current dir.
#[tokio::main]
async fn main() {
    // let text equal every command line argument after the name of the function.
    let mut text = String::new();
    for arg in std::env::args().skip(1) {
        text.push_str(&arg);
        text.push(' ');
    }
    // Remove the last space.
    text.pop();

    let client = Client::new();
    // let system_instructions = tokio::fs::read_to_string("systeminstruction.txt").await.expect("Unable to read system instructions.");
    let system_instructions = "You generate command line commands for linux.";
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-4")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_instructions)
                .build().unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(text.clone())
                .build().unwrap()
                .into(),
        ])
        .build().unwrap();
    let response = client.chat().create(request).await;
    if let Err(ref e) = response {
        println!("Error: {}", e);
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    let response = response.unwrap();
    let mut text = String::new();
    for message in response.choices.iter().take(1) {
        if let Some(content) = message.message.content.as_ref() {
            text.push_str(content);
        }
    }
    println!("{}", text);
}