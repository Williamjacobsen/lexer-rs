#[path = "handwritten-lexer.rs"]
mod handwritten_lexer;

#[path = "DFA-based-lexer.rs"]
mod DFA_based_lexer;

fn main() {
    let input = r#"Print("Test"); Some_variable="abc"; Print(123);"#;

    // handwritten lexer:
    println!("Handwritten lexer:");
    let mut lexer = handwritten_lexer::Lexer::new(input);
    lexer.parse_input();

    // DFA-based lexer:
    println!("\nDFA-based lexer:");
    let mut lexer = DFA_based_lexer::Lexer::new(input);
    lexer.parse_input();
}
