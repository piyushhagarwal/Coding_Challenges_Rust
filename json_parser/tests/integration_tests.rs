use json_parser::lexer::lexer;
use json_parser::parser::parse;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json1() {
        let json = "{";
        assert_eq!(parse(&lexer(json)), false);
    }

    #[test]
    fn test_json2() {
        let json = "}";
        assert_eq!(parse(&lexer(json)), false);
    }

    #[test]
    fn test_json3() {
        let json = "{}";
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json4() {
        let json = r#"{"key": "value",}"#; // Invalid JSON
        assert_eq!(parse(&lexer(json)), false);
    }

    #[test]
    fn test_json5() {
        let json = r#"
            {
                "key": "value",
                key2: "value"
            }"#; 
        assert_eq!(parse(&lexer(json)), false);
    }

    #[test]
    fn test_json6() {
        let json = r#"
            {
                "key": "value"
            }"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json7() {
        let json = r#"
            {
                "key": "value",
                "key2": "value"
            }"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json8() {
        let json = r#"
            {
                "key1": true,
                "key2": False,
                "key3": null,
                "key4": "value",
                "key5": 101
            }"#; 
        assert_eq!(parse(&lexer(json)), false);
    }

    #[test]
    fn test_json9() {
        let json = r#"
            {
                "key1": true,
                "key2": false,
                "key3": null,
                "key4": "value",
                "key5": 101
            }"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json10() {
        let json = r#"
            {
                "key": "value",
                "key-n": 101,
                "key-o": {
                    "inner key": "inner value"
                },
                "key-l": ['list value']
            }"#; // Invalid JSON
        assert_eq!(parse(&lexer(json)), false);
    }

    #[test]
    fn test_json11() {
        let json = r#"
            {
                "key": "value",
                "key-n": 101,
                "key-o": {},
                "key-l": []
            }"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json12() {
        let json = r#"
            {
                "key": "value",
                "key-n": 101,
                "key-o": {
                    "inner key": "inner value"
                },
                "key-l": ["list value"]
            }"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json13() {
        let json = r#"
            [
                "value1",
                "value2",
                "value3"
            ]"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }

    #[test]
    fn test_json14() {
        let json = r#"
            {
                "name": "John",
                "age": 30,
                "car": null,
                "hobbies": [],
                "isMarried": false
            }"#; // Valid JSON
        assert_eq!(parse(&lexer(json)), true);
    }
}
