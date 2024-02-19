// Token Enum tells the type of token we can have in JSON

#[derive(Debug, PartialEq)] //Debug trait is used to print the enum and PartialEq is used to compare the enum
pub enum TokenType {
    OpenBrace,
    CloseBrace,
    OpenBracket, // [ for array
    CloseBracket,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
}

pub struct Token{
    pub token: TokenType,
    pub value: String,
}

pub fn lexer(json: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_pointer = 0;

    while current_pointer < json.len() {
        if json.chars().nth(current_pointer) == Some('{') {
            tokens.push(Token{token: TokenType::OpenBrace, value: String::from("{")});
            current_pointer += 1;
        }
        else if json.chars().nth(current_pointer) == Some('}') {
            tokens.push(Token{token: TokenType::CloseBrace, value: String::from("}")});
            current_pointer += 1;
        }
        else if json.chars().nth(current_pointer) == Some('[') {
            tokens.push(Token{token: TokenType::OpenBracket, value: String::from("[")});
            current_pointer += 1;
        }
        else if json.chars().nth(current_pointer) == Some(']') {
            tokens.push(Token{token: TokenType::CloseBracket, value: String::from("]")});
            current_pointer += 1;
        }
        else if json.chars().nth(current_pointer) == Some(':') {
            tokens.push(Token{token: TokenType::Colon, value: String::from(":")});
            current_pointer += 1;
        }
        else if json.chars().nth(current_pointer) == Some(',') {
            tokens.push(Token{token: TokenType::Comma, value: String::from(",")});
            current_pointer += 1;
        }
        else if json.chars().nth(current_pointer).expect("Index out of bound").is_numeric() {
            let mut value = String::new();
            let mut i = current_pointer;
            while json.chars().nth(i).expect("Index out of bound").is_numeric() {
                value.push(json.chars().nth(i).expect("Index out of bound"));
                i += 1;
            }
            tokens.push(Token{token: TokenType::Number, value});
            current_pointer = i;
        }
        else if json.chars().nth(current_pointer) == Some('t') {
            let mut value = String::new();
            let mut i = current_pointer;
            if json.chars().nth(i) == Some('t') && json.chars().nth(i+1) == Some('r') && json.chars().nth(i+2) == Some('u') && json.chars().nth(i+3) == Some('e')  {
                i += 4;
                value = String::from("true");
            }
            tokens.push(Token{token: TokenType::True, value});
            current_pointer = i;
        }
        else if json.chars().nth(current_pointer) == Some('f') {
            let mut value = String::new();
            let mut i = current_pointer;
            if json.chars().nth(i) == Some('f') && json.chars().nth(i+1) == Some('a') && json.chars().nth(i+2) == Some('l') && json.chars().nth(i+3) == Some('s') && json.chars().nth(i+4) == Some('e') {
                i += 5;
                value = String::from("false");
            }
            tokens.push(Token{token: TokenType::False, value});
            current_pointer = i;
        }
        else if json.chars().nth(current_pointer) == Some('n') {
            let mut value = String::new();
            let mut i = current_pointer;
            if json.chars().nth(i) == Some('n') && json.chars().nth(i+1) == Some('u') && json.chars().nth(i+2) == Some('l') && json.chars().nth(i+3) == Some('l')  {
                i += 4;
                value = String::from("null");
            }
            tokens.push(Token{token: TokenType::Null, value});
            current_pointer = i;
        }
        else if json.chars().nth(current_pointer) == Some('"') {
            let mut value = String::new();
            let mut i = current_pointer + 1;
            while json.chars().nth(i) != Some('"') {
                value.push(json.chars().nth(i).expect("Index out of bound"));
                i += 1;
            }
            tokens.push(Token{token: TokenType::String, value});
            current_pointer = i + 1;
        }
        else if json.chars().nth(current_pointer) == Some(' ') || json.chars().nth(current_pointer) == Some('\n') || json.chars().nth(current_pointer) == Some('\t'){
            current_pointer += 1;
        }
        else {
            panic!("Invalid argument passed to the lexer function. {} .", json.chars().nth(current_pointer).expect("Index out of bound"));
        }
    }
    tokens // Return the tokens vector
}


// Test cases for the lexer function
// "cargo test" command is used to run the test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let json = r#"
        {
            "name": "John",
            "age": 30,
            "car": null,
            "hobbies": [],
            "isMarried": false
        }"#;

        
        let tokens = lexer(json);
        assert_eq!(tokens.len(), 22);
        assert_eq!(tokens[0].token, TokenType::OpenBrace);
        assert_eq!(tokens[1].token, TokenType::String);
        assert_eq!(tokens[2].token, TokenType::Colon);
        assert_eq!(tokens[3].token, TokenType::String);
        assert_eq!(tokens[4].token, TokenType::Comma);
        assert_eq!(tokens[5].token, TokenType::String);
        assert_eq!(tokens[6].token, TokenType::Colon);
        assert_eq!(tokens[7].token, TokenType::Number);
        assert_eq!(tokens[8].token, TokenType::Comma);
        assert_eq!(tokens[9].token, TokenType::String);
        assert_eq!(tokens[10].token, TokenType::Colon);
        assert_eq!(tokens[11].token, TokenType::Null);
        assert_eq!(tokens[12].token, TokenType::Comma);
        assert_eq!(tokens[13].token, TokenType::String);
        assert_eq!(tokens[14].token, TokenType::Colon);
        assert_eq!(tokens[15].token, TokenType::OpenBracket);
        assert_eq!(tokens[16].token, TokenType::CloseBracket);
        assert_eq!(tokens[17].token, TokenType::Comma);
        assert_eq!(tokens[18].token, TokenType::String);
        assert_eq!(tokens[19].token, TokenType::Colon);
        assert_eq!(tokens[20].token, TokenType::False);
        assert_eq!(tokens[21].token, TokenType::CloseBrace);
    }
}

//     OpenBrace : {
//     String : name
//     Colon : :
//     String : John
//     Comma : ,
//     String : age
//     Colon : :
//     Number : 30
//     Comma : ,
//     String : car
//     Colon : :
//     Null : null
//     Comma : ,
//     String : hobbies
//     Colon : :
//     OpenBracket : [
//     CloseBracket : ]
//     Comma : ,
//     String : isMarried
//     Colon : :
//     False : false
//     CloseBrace : }