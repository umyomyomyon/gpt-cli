use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs,
        Role,
        ChatCompletionRequestMessage,
        CreateChatCompletionResponse,
    },
    Client,
};

pub async fn request_chatgpt(input: &String) -> Result<CreateChatCompletionResponse , Box<dyn std::error::Error>> {
    let messages = make_messages(&input);
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(521u16)
        .model("gpt-3.5-turbo")
        .messages(messages.unwrap())
        .build()?;
    let response = client.chat().create(request).await?;
    Ok::<CreateChatCompletionResponse, Box<dyn std::error::Error>>(response)
}

fn make_messages(input: &String) -> Result<Vec<ChatCompletionRequestMessage>, Box<dyn std::error::Error>>{
    Ok(vec![
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content("You are a helpful assistant.")
            .build()?,
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(input)
            .build()?,
    ])
}
