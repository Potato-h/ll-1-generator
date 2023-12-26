use logos::{Lexer, Logos};
use std::fmt; // to implement the Display trait

fn literal(lex: &mut Lexer<Token>) -> String {
    let mut res: String = lex.slice().chars().skip(1).collect();
    res.pop(); // pop closing "
    res
}

fn code(lex: &mut Lexer<Token>) -> String {
    while !lex.slice().ends_with("}!") {
        let byte_len = lex.remainder().chars().next().map_or(1, |ch| ch.len_utf8());
        lex.bump(byte_len);
    }

    let mut res: String = lex.slice().chars().skip(2).collect();
    res.pop();
    res.pop();
    res
}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex("\"([^\"]|(\\\\\"))*\"", literal)]
    Literal(String),

    #[regex("!\\{", code)]
    Code(String),

    #[token(",")]
    Comma,

    #[token("preamble")]
    Preamble,
    #[token("tokens")]
    Tokens,
    #[token("rules")]
    Rules,

    #[token("pub")]
    Pub,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,

    #[token("=")]
    Assign,
    #[token(":")]
    Colon,
    #[token("=>")]
    Arrow,
    #[token("->")]
    TyArrow,

    #[token("token")]
    Tok,
    #[token("regex")]
    Reg,

    #[regex(r"#.*\n?", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
