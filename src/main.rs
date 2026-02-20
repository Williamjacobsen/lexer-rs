#[derive(Debug)]
enum Token {
    Print,
    LeftParen,
    RightParen,
    Equal,
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

    pub fn parse_input(&mut self) {
        while self.position < self.input.len() {
            let token = self.next_token();
            println!("{:?}", token);
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.input.len() {
            self.current_char = self.input.as_bytes()[self.position];
        }
    }

    fn next_token(&mut self) -> Token {
        let token = match self.current_char {
            b' ' => Token::Illegal, // temporary
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'=' => Token::Equal,
            b';' => Token::SemiColon,
            b'"' => {
                // Consume string,
                Token::Illegal // temporary
            }
            b'0'..=b'9' => {
                // Consume integer,
                Token::Illegal // temporary
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier();
            }
            _ => Token::Illegal,
        };

        self.advance();

        return token;
    }

    fn consume_identifier(&mut self) -> Token {
        let starting_position = self.position;

        loop {
            match self.current_char {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.advance(),
                _ => break,
            }
        }

        let ending_position = self.position;

        Token::String(self.input[starting_position..ending_position].to_string())
    }
}

fn main() {
    let input = r#"Print("Test"); Some_variable="abc";"#;

    let mut lexer = Lexer::new(input);

    lexer.parse_input();

    //for char in input.chars() {
    //    println!("{:?}", lexer.next_token());
    //}
}
