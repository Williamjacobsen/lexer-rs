#[path = "Ad-hoc-lexer.rs"]
mod ad_hoc_lexer;

#[path = "DFA-based-lexer.rs"]
mod dfa_based_lexer;

fn main() {
    let input = r#"Print("Test"); Some_variable="abc"; Print(123);"#;

    // Ad-Hoc lexer:
    println!("Ad-Hoc lexer:");
    let mut lexer = ad_hoc_lexer::Lexer::new(input);
    lexer.parse_input();

    // DFA-based lexer:
    println!("\nDFA-based lexer:");
    let mut lexer = dfa_based_lexer::Lexer::new(input);
    lexer.parse_input();
}
