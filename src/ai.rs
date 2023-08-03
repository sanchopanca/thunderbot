use anyhow::Result;
use chat_gpt_lib_rs::{self, ChatGPTClient, ChatInput, Message, Model, Role};
use std::env;

#[allow(dead_code)]
pub async fn ask_ai_for_summarization(messages: String) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY").expect("Provide OPENAI_API_KEY env variable");

    let client = ChatGPTClient::new(&api_key, "https://api.openai.com");

    let chat_input = ChatInput {
        model: Model::Gpt3_5Turbo,
        messages: vec![
            Message {
                role: Role::System,
                content: "You are summarizing last 50 messages in group chat. Be not too verbose and use informal language".to_string(),
            },
            Message {
                role: Role::User,
                content: messages,
            },
        ],
        ..Default::default()
    };

    match client.chat(chat_input).await {
        Ok(response) => {
            println!("{:?}", response);
            Ok(response.choices[0].message.content.clone())
        }
        Err(e) => Err(e.into()),
    }
}
