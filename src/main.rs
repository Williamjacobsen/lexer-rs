#[derive(Debug)]
enum Token {
    Print,
    LeftParen,
    RightParen,
    String(String),
    Int(i64),
    SemiColon,
    Illegal,
}

pub struct Lexer<'a> {
    input: &'a str,
    current_char: u8,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            current_char: input.as_bytes()[0],
            position: 0,
        };

        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.input.len() {
            self.current_char = self.input.as_bytes()[self.position];
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.current_char {
            b' ' => {
                println!("Whitespace");
                Token::Illegal // temporary
            }
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b';' => Token::SemiColon,
            b'"' => {
                // Consume string,
                Token::Illegal // temporary
            }
            b'0'..=b'9' => {
                // Consume integer,
                Token::Illegal // temporary
            }
            //b'a'..=b'z' | b'A'..='Z' | b'_' => {
            //    // Consume identifier
            //}
            _ => Token::Illegal,
        };

        self.advance();

        return token;
    }
}

fn main() {
    let input = r#"Print("Test");"#;

    let mut lexer = Lexer::new(input);

    for char in input.chars() {
        println!("{:?}", lexer.next_token());
    }
}
