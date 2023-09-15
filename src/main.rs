mod lexer;
mod parser;

fn main() {
    let file_path = "test.vi";

    let input = lexer::read_file(file_path);
    println!("===================================================================================");
    println!();
    lexer::run_lexer(&input);
    println!();
    println!("===================================================================================");
}
