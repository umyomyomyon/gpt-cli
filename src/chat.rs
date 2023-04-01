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

pub fn check_apikey() {
    match std::env::var("OPENAI_API_KEY") {
        Ok(_) => (),
        Err(_) => {
            println!("Please set OPENAI_API_KEY env var: export OPENAI_API_KEY=YOUR_API_KEY");
            std::process::exit(1);
        },
    }
}

pub struct Chat {
    system_text: String,
    pub messages: Vec<ChatCompletionRequestMessage>,
}

impl Chat {
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

impl Chat {
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

    fn push_as_assistant(&mut self, content: &String) {
        self.push(Role::Assistant, content);
    }
}

impl Chat {
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

impl Chat {
    pub fn history(&self) -> String {
        let mut history = String::new();
        for message in &self.messages {
            history.push_str(&format!("{}: {}\n", message.role, message.content));
        }
        return history;
    }
}

impl Chat {
    pub async fn request_chatgpt(&mut self, model: Option<String>) -> Result<CreateChatCompletionResponse, Box<dyn std::error::Error>> {
        let client = Client::new();
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.choose_model(model))
            .messages(self.messages.clone())
            .build()?;
        let response = client.chat().create(request).await?;
        self.push_as_assistant(&response.choices[0].message.content);
        Ok::<CreateChatCompletionResponse, Box<dyn std::error::Error>>(response)
    }

    fn choose_model(&self, model: Option<String>) -> String {
        if model.is_some() {
            model.unwrap()
        } else {
            "gpt-3.5-turbo".to_string()
        }
    }
}
