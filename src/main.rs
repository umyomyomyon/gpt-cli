mod cli;
mod api;

use clap::Parser;
use cli::Cli;
use api::Messages;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut messages = Messages::new(None);
    loop {
        let input = cli::prompt();
        if input == "exit" {
            cli::exit_message();
            break;
        } else if input == "clear" {
            messages.clear();
            continue;
        } else if input == "history" {
            let history = messages.history();
            println!("{}", history);
            continue;
        }
        messages.push_as_user(&input);
        let result =
            api::request_chatgpt(messages.messages.clone(), args.model.clone()).await.unwrap();
        messages.push_as_assistant(&result.choices[0].message.content);
        println!("{}", result.choices[0].message.content);
    }
}
