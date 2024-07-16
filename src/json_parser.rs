use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{
    json_object::Object as JsonObject,
    json_tokenizer::{Token, Tokenizer},
};

#[derive(Debug)]
pub enum ParserError {
    Empty,
    InvalidObject,
    InvalidString,
    InvalidKeyValueSeparator,
    InvalidFieldSeparator,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "JSON parse error!")
    }
}

#[derive(PartialEq)]
enum ParserState {
    Default,
    Object,
    Key,
    Value,
    ExpectingKey,
}

pub struct Parser {
    state: ParserState,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: ParserState::Default,
        }
    }

    pub fn parse(mut self, json_string: &str) -> Result<JsonObject, ParserError> {
        let mut tokenizer: Tokenizer = Tokenizer::new(json_string);
        let mut next_token = tokenizer.get_next();

        if next_token.is_none() {
            return Err(ParserError::Empty);
        }

        while let Some(ref token) = next_token {
            match token {
                Token::OpenObject => self.state = ParserState::Object,
                Token::CloseObject => match self.state {
                    ParserState::Object => self.state = ParserState::Default,
                    _ => return Err(ParserError::InvalidObject),
                },
                Token::String(_) => match self.state {
                    ParserState::Object => self.state = ParserState::Key,
                    ParserState::ExpectingKey => self.state = ParserState::Key,
                    ParserState::Value => self.state = ParserState::Object,
                    _ => return Err(ParserError::InvalidString),
                },
                Token::KeyValueSeparator => match self.state {
                    ParserState::Key => self.state = ParserState::Value,
                    _ => return Err(ParserError::InvalidKeyValueSeparator),
                },
                Token::FieldSeparator => match self.state {
                    ParserState::Object => self.state = ParserState::ExpectingKey,
                    _ => return Err(ParserError::InvalidFieldSeparator),
                },
            }

            next_token = tokenizer.get_next();
        }

        if self.state != ParserState::Default {
            Err(ParserError::InvalidObject)
        } else {
            Ok(JsonObject {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_object_success() {
        // GIVEN:
        let empty_object: &str = "{}";
        let parser: Parser = Parser::new();

        // WHEN:
        let result: Result<JsonObject, ParserError> = parser.parse(empty_object);

        // THEN:
        assert!(result.is_ok());
    }

    #[test]
    fn parse_empty_string_failure() {
        // GIVEN:
        let empty_string: &str = "";
        let parser: Parser = Parser::new();

        // WHEN:
        let result: Result<JsonObject, ParserError> = parser.parse(empty_string);

        // THEN:
        assert!(result.is_err());
    }

    #[test]
    fn parse_trailing_comma_failure() {
        // GIVEN:
        let json_string: &str = r#"{"key": "value",}"#;
        let parser: Parser = Parser::new();

        // WHEN:
        let result: Result<JsonObject, ParserError> = parser.parse(json_string);

        // THEN:
        assert!(result.is_err());
    }

    #[test]
    fn parse_unquoted_key_failure() {
        // GIVEN:
        let json_string: &str = r#"{
  "key": "value",
  key2: "value"
}
"#;
        let parser: Parser = Parser::new();

        // WHEN:
        let result: Result<JsonObject, ParserError> = parser.parse(json_string);

        // THEN:
        assert!(result.is_err());
    }

    #[test]
    fn parse_object_key_value_pair_success() {
        // GIVEN:
        let json_string: &str = r#"{"key": "value"}"#;
        let parser: Parser = Parser::new();

        // WHEN:
        let result: Result<JsonObject, ParserError> = parser.parse(json_string);

        // THEN:
        assert!(result.is_ok());
    }

    #[test]
    fn parse_object_multiple_key_value_pair_success() {
        // GIVEN:
        let json_string: &str = r#"{
            "key": "value",
            "key2": "value"
          }
          "#;
        let parser: Parser = Parser::new();

        // WHEN:
        let result: Result<JsonObject, ParserError> = parser.parse(json_string);

        // THEN:
        assert!(result.is_ok());
    }
}
