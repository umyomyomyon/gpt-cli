use std::io::Write;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub model: Option<String>
}

pub fn prompt() -> String {
    print_head();
    let input = read_input();
    return input;
}

fn print_head() {
    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read line");
    return line.trim().to_string();
}

pub fn exit_message() {
    println!("Goodbye!");
}

pub fn please_input() {
    println!("Please input something.");
}

pub fn check_apikey() {
    match std::env::var("OPENAI_API_KEY") {
        Ok(_) => (),
        Err(_) => {
            println!("Please set OPENAI_API_KEY env var: export OPENAI_API_KEY=YOUR_API_KEY");
            std::process::exit(1);
        },
    }
}
