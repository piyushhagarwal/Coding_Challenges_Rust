mod lexer;
mod parser;
use lexer::lexer;
use parser::parse;
fn main() {
    let json1 = "{";
    let json2 = r#"
    {
        "name": "John",
        "age": 30,
        "car": null,
        "hobbies": [],
        "isMarried": false
    }"#;
    let tokens = lexer(json1);
    let rusult = parse(&tokens);

    // Print the tokens
    for token in &tokens {
        println!("{:?} : {}", token.token_type, token.value);
        println!("Result: {}", rusult);
    }
}

