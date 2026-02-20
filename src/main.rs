#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    Print,
    LeftParen,
    RightParen,
    Equal,
    String(String),
    Int(i64),
    Identifier(String),
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
        let lexer = Lexer {
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

        self.skip_whitespaces();
    }

    fn skip_whitespaces(&mut self) {
        while self.position < self.input.len()
            && (self.current_char == b' ' || self.current_char == b'\n')
        {
            self.advance();
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let token = match self.current_char {
            b' ' => Token::Illegal, // temporary
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'=' => Token::Equal,
            b';' => Token::SemiColon,
            b'"' => {
                return self.consume_string();
            }
            b'0'..=b'9' => {
                return self.consume_integer();
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

        let literal = &self.input[starting_position..ending_position];

        match literal {
            "Print" => Token::Print,
            _ => Token::Identifier(literal.to_string()),
        }
    }

    fn consume_string(&mut self) -> Token {
        let starting_position = self.position;

        loop {
            match self.current_char {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'"' => self.advance(),
                _ => break,
            }
        }

        let ending_position = self.position;

        Token::String(self.input[starting_position + 1..ending_position - 1].to_string())
    }

    fn consume_integer(&mut self) -> Token {
        let starting_position = self.position;

        loop {
            match self.current_char {
                b'0'..=b'9' => self.advance(),
                _ => break,
            }
        }

        let ending_position = self.position;

        let literal = &self.input[starting_position..ending_position];

        let value = literal.parse::<i64>().unwrap_or(0);

        Token::Int(value)
    }
}

/*
Input:
Print("Test"); Some_variable="abc"; Print(123);

Output:

Print
LeftParen
String("Test")
RightParen
SemiColon
Identifier("Some_variable")
Equal
String("abc")
SemiColon
Print
LeftParen
Int(123)
RightParen
SemiColon
 */

fn main() {
    let input = r#"Print("Test"); Some_variable="abc"; Print(123);"#;

    let mut lexer = Lexer::new(input);

    lexer.parse_input();
}
