use std::io::Write;

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
