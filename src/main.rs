mod lexer;

use lexer::Lexer;

fn main() {
    let code = std::fs::read_to_string("test.rn").unwrap();
    let l = Lexer::new(&code);
    let res = l.tokenize();
    println!("{:#?}", res);
}
