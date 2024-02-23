// Token Enum tells the type of token we can have in JSON

#[derive(Debug, PartialEq)] //Debug trait is used to print the enum and PartialEq is used to compare the enum
pub enum TokenType {
    // Structural Tokens
    OpenBrace,      // {
    CloseBrace,     // }
    OpenBracket,    // [
    CloseBracket,   // ]
    Colon,          // :
    Comma,          // ,
    
    // Value Tokens
    String,         // "string"
    Number,         // 123 or 12.34
    True,           // true
    False,          // false
    Null,           // null
}

pub struct Token{
    pub token_type: TokenType,
    pub value: String,
}

pub fn lexer(json: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_pointer = 0;

    // Loop through the input JSON string
    while current_pointer < json.len() {
        // Check for opening curly brace '{'
        if json.chars().nth(current_pointer) == Some('{') {
            tokens.push(Token{token_type: TokenType::OpenBrace, value: String::from("{")});
            current_pointer += 1;
        }
        // Check for closing curly brace '}'
        else if json.chars().nth(current_pointer) == Some('}') {
            tokens.push(Token{token_type: TokenType::CloseBrace, value: String::from("}")});
            current_pointer += 1;
        }
        // Check for opening square bracket '['
        else if json.chars().nth(current_pointer) == Some('[') {
            tokens.push(Token{token_type: TokenType::OpenBracket, value: String::from("[")});
            current_pointer += 1;
        }
        // Check for closing square bracket ']'
        else if json.chars().nth(current_pointer) == Some(']') {
            tokens.push(Token{token_type: TokenType::CloseBracket, value: String::from("]")});
            current_pointer += 1;
        }
        // Check for colon ':'
        else if json.chars().nth(current_pointer) == Some(':') {
            tokens.push(Token{token_type: TokenType::Colon, value: String::from(":")});
            current_pointer += 1;
        }
        // Check for comma ','
        else if json.chars().nth(current_pointer) == Some(',') {
            tokens.push(Token{token_type: TokenType::Comma, value: String::from(",")});
            current_pointer += 1;
        }
        // Check for numeric values
        else if json.chars().nth(current_pointer).expect("Index out of bound").is_numeric() {
            let mut value = String::new();
            let mut i = current_pointer;
            // Collect numeric characters
            while json.chars().nth(i).expect("Index out of bound").is_numeric() {
                value.push(json.chars().nth(i).expect("Index out of bound"));
                i += 1;
            }
            tokens.push(Token{token_type: TokenType::Number, value});
            current_pointer = i;
        }
        // Check for the keyword 'true'
        else if json.chars().nth(current_pointer) == Some('t') {
            let mut value = String::new();
            let mut i = current_pointer;
            // Verify 'true' and update index
            if json.chars().nth(i) == Some('t') && json.chars().nth(i+1) == Some('r') && json.chars().nth(i+2) == Some('u') && json.chars().nth(i+3) == Some('e')  {
                i += 4;
                value = String::from("true");
            }
            tokens.push(Token{token_type: TokenType::True, value});
            current_pointer = i;
        }
        // Check for the keyword 'false'
        else if json.chars().nth(current_pointer) == Some('f') {
            let mut value = String::new();
            let mut i = current_pointer;
            // Verify 'false' and update index
            if json.chars().nth(i) == Some('f') && json.chars().nth(i+1) == Some('a') && json.chars().nth(i+2) == Some('l') && json.chars().nth(i+3) == Some('s') && json.chars().nth(i+4) == Some('e') {
                i += 5;
                value = String::from("false");
            }
            tokens.push(Token{token_type: TokenType::False, value});
            current_pointer = i;
        }
        // Check for the keyword 'null'
        else if json.chars().nth(current_pointer) == Some('n') {
            let mut value = String::new();
            let mut i = current_pointer;
            // Verify 'null' and update index
            if json.chars().nth(i) == Some('n') && json.chars().nth(i+1) == Some('u') && json.chars().nth(i+2) == Some('l') && json.chars().nth(i+3) == Some('l')  {
                i += 4;
                value = String::from("null");
            }
            tokens.push(Token{token_type: TokenType::Null, value});
            current_pointer = i;
        }
        // Check for double quotes indicating the start of a string
        else if json.chars().nth(current_pointer) == Some('"') {
            let mut value = String::new();
            let mut i = current_pointer + 1;
            // Collect characters until the closing double quote
            while json.chars().nth(i) != Some('"') {
                value.push(json.chars().nth(i).expect("Index out of bound"));
                i += 1;
            }
            tokens.push(Token{token_type: TokenType::String, value});
            current_pointer = i + 1;
        }
        // Ignore whitespace characters
        else if json.chars().nth(current_pointer) == Some(' ') || json.chars().nth(current_pointer) == Some('\n') || json.chars().nth(current_pointer) == Some('\t'){
            current_pointer += 1;
        }
        // If the character is not a valid JSON token, return an empty vector
        else {
            return Vec::new();
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
        assert_eq!(tokens[0].token_type, TokenType::OpenBrace);
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::String);
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::String);
        assert_eq!(tokens[6].token_type, TokenType::Colon);
        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[8].token_type, TokenType::Comma);
        assert_eq!(tokens[9].token_type, TokenType::String);
        assert_eq!(tokens[10].token_type, TokenType::Colon);
        assert_eq!(tokens[11].token_type, TokenType::Null);
        assert_eq!(tokens[12].token_type, TokenType::Comma);
        assert_eq!(tokens[13].token_type, TokenType::String);
        assert_eq!(tokens[14].token_type, TokenType::Colon);
        assert_eq!(tokens[15].token_type, TokenType::OpenBracket);
        assert_eq!(tokens[16].token_type, TokenType::CloseBracket);
        assert_eq!(tokens[17].token_type, TokenType::Comma);
        assert_eq!(tokens[18].token_type, TokenType::String);
        assert_eq!(tokens[19].token_type, TokenType::Colon);
        assert_eq!(tokens[20].token_type, TokenType::False);
        assert_eq!(tokens[21].token_type, TokenType::CloseBrace);
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