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
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "JSON parse error!")
    }
}

enum ParserState {
    Default,
    Object,
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
                    ParserState::Object => {}
                    _ => {
                        return Err(ParserError::InvalidObject);
                    }
                },
            }

            next_token = tokenizer.get_next();
        }

        Ok(JsonObject {})
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
}
