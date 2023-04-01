mod cli;
mod chat;

use clap::Parser;
use cli::Cli;
use chat::Chat;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut chat = Chat::new(None);
    loop {
        let input = cli::prompt();
        if input == "exit" {
            cli::exit_message();
            break;
        } else if input == "clear" {
            chat.clear();
            continue;
        } else if input == "history" {
            let history = chat.history();
            println!("{}", history);
            continue;
        }
        chat.push_as_user(&input);
        let result =
            chat.request_chatgpt(args.model.clone()).await.unwrap();
        println!("{}", result.choices[0].message.content);
    }
}
