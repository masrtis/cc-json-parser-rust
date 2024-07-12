pub enum Token {
    OpenObject,
    CloseObject,
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
        let next_char = self.json_string.chars().nth(self.position);

        if let Some(current_char) = next_char {
            self.position += 1;
            return match current_char {
                '{' => Some(Token::OpenObject),
                '}' => Some(Token::CloseObject),
                _ => None
            };
        }

        None
    }
}
