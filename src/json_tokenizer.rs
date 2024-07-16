pub enum Token {
    OpenObject,
    CloseObject,
    String(String),
    KeyValueSeparator,
    FieldSeparator,
}

pub struct Tokenizer {
    json_string: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(json_string: &str) -> Self {
        Tokenizer {
            json_string: json_string.to_string(),
            position: 0,
        }
    }

    pub fn get_next(&mut self) -> Option<Token> {
        let current_position = self.position;

        let remaining_chars = self
            .json_string
            .char_indices()
            .filter(|(pos, _)| *pos >= current_position);

        for (_, next_char) in remaining_chars {
            self.position += 1;

            if next_char.is_whitespace() {
                continue;
            }

            return match next_char {
                '{' => Some(Token::OpenObject),
                '}' => Some(Token::CloseObject),
                '\"' => self.get_string(),
                ':' => Some(Token::KeyValueSeparator),
                ',' => Some(Token::FieldSeparator),
                _ => None,
            };
        }

        None
    }

    fn get_string(&mut self) -> Option<Token> {
        let current_position = self.position;

        let remaining_chars = self
            .json_string
            .char_indices()
            .filter(|(pos, _)| *pos >= current_position);

        let mut token_string: String = String::new();
        let mut end_string_found: bool = false;

        for (_, curr) in remaining_chars {
            self.position += 1;

            if curr == '\"' {
                end_string_found = true;
                break;
            }

            token_string.push(curr);
        }

        if end_string_found {
            Some(Token::String(token_string))
        } else {
            None
        }
    }
}
