mod cli;
mod api;

#[tokio::main]
async fn main() {
    loop {
        let input = cli::prompt();
        let result = api::request_chatgpt(&input).await.unwrap();
        println!("{}", result.choices[0].message.content);
    }
}
