mod lexer;
mod parser;
mod rvm;

use lexer::Lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if let Some(filename) = args.get(1) {
        filename
    }
    else {
        println!("where is filename? stupid pig");
        std::process::exit(1);
    };
    let code = std::fs::read_to_string(filename).unwrap();
    let r = rvm::asm::assemble(&code);
    println!("{:?}", r)
}
