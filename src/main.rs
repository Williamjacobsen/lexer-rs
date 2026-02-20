enum Token {
    Print,
    LeftParen,
    RightParen,
    String(String),
    Int(i64),
    SemiColon,
}

struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer { input };

        return lexer;
    }

    pub fn next_token() -> Token {
        let token = Token::SemiColon;
        return token;
    }
}

fn main() {
    println!("Hello, world!");

    let input = r#"Print("Test");"#;

    let mut lexer = Lexer::new(input);
}
