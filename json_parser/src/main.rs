mod lexer;
use lexer::lexer;
fn main() {
    let json = r#"
    {
        "name": "John",
        "age": 30,
        "car": null,
        "hobbies": [],
        "isMarried": false
    }"#;
    let tokens = lexer(json);

    // Print the tokens
    for token in &tokens {
        println!("{:?} : {}", token.token, token.value);
    }
}

