mod cli;
mod api;

use api::Messages;

#[tokio::main]
async fn main() {
    let mut messages = Messages::new(None);
    loop {
        let input = cli::prompt();
        if input == "exit" {
            cli::exit_message();
            break;
        }
        messages.push_as_user(&input);
        let result = api::request_chatgpt(messages.messages.clone()).await.unwrap();
        messages.push_as_assistant(&result.choices[0].message.content);
        println!("{}", result.choices[0].message.content);
    }
}
