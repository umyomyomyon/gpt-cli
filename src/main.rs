mod cli;
mod chat;
mod spinner;

use tokio::runtime;
use clap::Parser;
use cli::Cli;
use chat::Chat;

fn main() {
    let args = Cli::parse();
    let mut chat = Chat::new(None);
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build runtime");
    loop {
        let input = cli::prompt();
        match input.as_str() {
            "exit" => {
                cli::exit_message();
                break;
            },
            "clear" => {
                chat.clear();
                continue;
            },
            "history" => {
                let history = chat.history();
                println!("{}", history);
                continue;
            },
            "" => {
                cli::please_input();
                continue;
            },
            _ => {
                chat.push_as_user(&input);
                let spinner = spinner::init();
                let future = async {
                    let result =
                        chat.request_chatgpt(args.model.clone()).await.unwrap();
                    spinner.finish_and_clear();
                    println!("{}", result.choices[0].message.content);
                };
                rt.block_on(future);
            },
        }
    }
}
