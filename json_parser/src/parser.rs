// Import the `Token` enum from the `lexer` module
use crate::lexer::Token;

// Import the `TokenType` enum from the `lexer` module
use crate::lexer::TokenType;

// Function to parse the tokens vector and check if the JSON is valid
pub fn parse(tokens: &Vec<Token>) -> bool {
    // Check if the tokens vector is empty
    if tokens.len() == 0 {
        // If the JSON is empty, return false
        return false;
    }

    let mut current_pointer = 0;

    // Call the recursive parse_value function to start parsing the JSON
    let result = parse_value(&tokens, &mut current_pointer);
    
    // Return true if parsing is successful and the entire tokens vector is consumed
    result && current_pointer == tokens.len()
}


// Function to parse individual values or in our case individual tokens
pub fn parse_value(tokens : &Vec<Token>, current_pointer : &mut usize) -> bool{
    let token = &tokens[*current_pointer];
    if token.token_type == TokenType::String {
        true
    } else if token.token_type == TokenType::Number {
        true
    } else if token.token_type == TokenType::True {
        true
    } else if token.token_type == TokenType::False {
        true
    } else if token.token_type == TokenType::Null {
        true
    } else if token.token_type == TokenType::OpenBrace {
        parse_object(tokens, current_pointer)
    } else if token.token_type == TokenType::OpenBracket {
        parse_array(tokens, current_pointer)
    } else{
        false
    }
}

// Fuction to parse and check if the object is valid
pub fn parse_object(tokens : &Vec<Token>, current_pointer : &mut usize) -> bool{
    *current_pointer += 1; // Skip the open brace
    while *current_pointer < tokens.len() && tokens[*current_pointer].token_type != TokenType::CloseBrace{
        
        if tokens[*current_pointer].token_type != TokenType::String{
            return false; // Key is not in the form of a string
        }

        *current_pointer += 1; // Skip the key

        if tokens[*current_pointer].token_type != TokenType::Colon{
            return false; // Key is not followed by a colon
        }

        *current_pointer += 1; // Skip the colon

        if !parse_value(tokens, current_pointer){
            return false;
        }

        *current_pointer += 1; // Skip the value
        
        // If the next token is a comma and after the comma there is closing brace, return false (e.g. {"key": "value",})
        if *current_pointer < tokens.len() && tokens[*current_pointer].token_type == TokenType::Comma && tokens[*current_pointer + 1].token_type == TokenType::CloseBrace{
            return false;
        }

        // If the next token is a comma, skip it
        if *current_pointer < tokens.len() && tokens[*current_pointer].token_type == TokenType::Comma{
            *current_pointer += 1; // Skip the comma
        }
    }

    if *current_pointer < tokens.len() && tokens[*current_pointer].token_type != TokenType::CloseBrace{
        return false; // If the object is not closed
    }

    *current_pointer += 1; // Skip the closing brace

    true // If the object is valid and current_pointer is at the closing brace
}

// Fuction to parse and check if the array is valid
pub fn parse_array(tokens : &Vec<Token>, current_pointer : &mut usize) -> bool{
    *current_pointer += 1; // Skip the open bracket
    while *current_pointer < tokens.len() && tokens[*current_pointer].token_type != TokenType::CloseBracket{
        if !parse_value(tokens, current_pointer){
            return false;
        }

        *current_pointer += 1; // Skip the value
        
        // If the next token is a comma and after the comma there is closing bracket, return false (e.g. ["value",])
        if *current_pointer < tokens.len() && tokens[*current_pointer].token_type == TokenType::Comma && tokens[*current_pointer + 1].token_type == TokenType::CloseBracket{
            return false;
        }

        // If the next token is a comma, skip it
        if *current_pointer < tokens.len() && tokens[*current_pointer].token_type == TokenType::Comma{
            *current_pointer += 1; // Skip the comma
        }
    }

    if *current_pointer < tokens.len() && tokens[*current_pointer].token_type != TokenType::CloseBracket{
        return false; // If the array is not closed
    }

    // If the json consists only array
    if *current_pointer == &tokens.len() - 1 {
        *current_pointer += 1;
    }

    true // If the array is valid
}