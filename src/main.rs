mod cli;
mod api;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let result = api::request_chatgpt(&args.message).await.unwrap();
    println!("{}", result.choices[0].message.content);
}
