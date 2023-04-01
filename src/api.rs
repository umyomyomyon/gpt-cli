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

pub async fn request_chatgpt(
    messages: Vec<ChatCompletionRequestMessage>,
    model: Option<String>
) -> Result<CreateChatCompletionResponse , Box<dyn std::error::Error>> {
    let model = if model.is_some() {
        model.unwrap()
    } else {
        "gpt-3.5-turbo".to_string()
    };
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .build()?;
    let response = client.chat().create(request).await?;
    Ok::<CreateChatCompletionResponse, Box<dyn std::error::Error>>(response)
}

pub struct Messages {
    system_text: String,
    pub messages: Vec<ChatCompletionRequestMessage>,
}

impl Messages {
    pub fn new(system_text: Option<&String>) -> Self {
        match system_text {
            Some(text) => Self {
                system_text: text.to_string(),
                messages: vec![
                    ChatCompletionRequestMessageArgs::default()
                        .role(Role::System)
                        .content(text)
                        .build()
                        .unwrap(),
                ],
            },
            None => Self {
                system_text: "You are a helpful assistant.".to_string(),
                messages: vec![
                    ChatCompletionRequestMessageArgs::default()
                        .role(Role::System)
                        .content("You are a helpful assistant.")
                        .build()
                        .unwrap(),
                ],
            },
        }
    }
}

impl Messages {
    fn push(&mut self, role: Role, content: &String) {
        self.messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(role)
                .content(content)
                .build()
                .unwrap(),
        );
    }

    pub fn push_as_user(&mut self, content: &String) {
        self.push(Role::User, content);
    }

    pub fn push_as_assistant(&mut self, content: &String) {
        self.push(Role::System, content);
    }
}

impl Messages {
    pub fn clear(&mut self) {
        self.messages = vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(&self.system_text)
                .build()
                .unwrap(),
        ];
    }
}

impl Messages {
    pub fn history(&self) -> String {
        let mut history = String::new();
        for message in &self.messages {
            history.push_str(&format!("{}: {}\n", message.role, message.content));
        }
        return history;
    }
}
