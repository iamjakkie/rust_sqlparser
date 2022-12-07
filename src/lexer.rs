use std::collections::HashMap;

// Define the tokens that the lexer will recognize
#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Keyword(String),
    Operator(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

// Define a map of keywords to their corresponding token
const KEYWORDS: HashMap<&str, Token> = [
    ("SELECT", Token::Keyword("SELECT".to_string())),
    ("FROM", Token::Keyword("FROM".to_string())),
    ("WHERE", Token::Keyword("WHERE".to_string())),
    ("AND", Token::Keyword("AND".to_string())),
    ("OR", Token::Keyword("OR".to_string())),
    ("NOT", Token::Keyword("NOT".to_string())),
    ("IS", Token::Keyword("IS".to_string())),
    ("NULL", Token::Keyword("NULL".to_string())),
    ("IN", Token::Keyword("IN".to_string())),
].iter().cloned().collect();

// Define a map of operators to their corresponding token
const OPERATORS: HashMap<&str, Token> = [
    ("+", Token::Operator("+".to_string())),
    ("-", Token::Operator("-".to_string())),
    ("*", Token::Operator("*".to_string())),
    ("/", Token::Operator("/".to_string())),
    ("%", Token::Operator("%".to_string())),
    ("^", Token::Operator("^".to_string())),
    ("=", Token::Operator("=".to_string())),
    ("!=", Token::Operator("!=".to_string())),
    ("<", Token::Operator("<".to_string())),
    (">", Token::Operator(">".to_string())),
    ("<=", Token::Operator("<=".to_string())),
    (">=", Token::Operator(">=".to_string())),
].iter().cloned().collect();

// Define a function to tokenize a SQL query
fn tokenize(query: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;
    let mut escape_next = false;
    let mut in_number = false;

    // Iterate over each character in the query
    for c in query.chars() {
        if in_string {
            // If we're in a string literal, append the character to the current token
            if escape_next {
                current_token.push(c);
                escape_next = false

