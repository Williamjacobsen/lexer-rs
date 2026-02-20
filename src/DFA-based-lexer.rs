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

#[derive(PartialEq)]
enum State {
    Start,
    InIndentifier,
    InString,
    InInteger,
    Done,
    Error,
}

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input: input.as_bytes(),
            position: 0,
        };

        lexer
    }

    pub fn parse_input(&mut self) {
        loop {
            while self.position < self.input.len()
                && matches!(self.input[self.position], b' ' | b'\n')
            {
                self.position += 1;
            }

            if self.position >= self.input.len() {
                break;
            }

            let token = self.next_token();
            println!("{:?}", token);
        }
    }

    fn next_token(&mut self) -> Token {
        let mut state = State::Start;
        let mut token_start = self.position;

        loop {
            let current_char = self.input[self.position];

            state = match (&state, current_char) {
                (State::Start, b'(') => {
                    self.position += 1;
                    return Token::LeftParen;
                }
                (State::Start, b')') => {
                    self.position += 1;
                    return Token::RightParen;
                }
                (State::Start, b'=') => {
                    self.position += 1;
                    return Token::Equal;
                }
                (State::Start, b';') => {
                    self.position += 1;
                    return Token::SemiColon;
                }

                // Handle Strings:
                (State::Start, b'"') => {
                    self.position += 1;
                    State::InString
                }
                (State::InString, b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                    self.position += 1;
                    State::InString
                }
                (State::InString, b'"') => {
                    self.position += 1;
                    State::Done
                }

                // Handle Numbers:
                (State::Start, b'0'..=b'9') => {
                    self.position += 1;
                    State::InInteger
                }
                (State::InInteger, b'0'..=b'9') => {
                    self.position += 1;
                    State::InInteger
                }
                (State::InInteger, _) => State::Done,

                // Handle Identifiers:
                (State::Start, b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                    self.position += 1;
                    State::InIndentifier
                }
                (State::InIndentifier, b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                    self.position += 1;
                    State::InIndentifier
                }
                (State::InIndentifier, _) => State::Done,

                (_, _) => {
                    self.position += 1;
                    State::Error
                }
            };

            if state == State::Done || state == State::Error {
                break;
            }
        }

        let slice = &self.input[token_start..self.position];

        match self.input[token_start] {
            b'"' => {
                let inner = &slice[1..slice.len() - 1];
                Token::String(String::from_utf8_lossy(inner).into_owned())
            }
            b'0'..=b'9' => Token::Int(
                str::from_utf8(slice)
                    .unwrap_or("0")
                    .parse::<i64>()
                    .unwrap_or(0),
            ),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = str::from_utf8(slice).unwrap_or("");
                match identifier {
                    "Print" => Token::Print,
                    _ => Token::Identifier(identifier.to_string()),
                }
            }

            _ => Token::Illegal,
        }
    }
}
