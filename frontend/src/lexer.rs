use logos::Logos;

#[derive(Logos, Debug)]
pub(crate) enum Token {
    #[regex("[ \t\r\n]+", logos::skip)]
    Whitespace,

    #[regex("//.*", logos::skip)]
    Comment,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"-?[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("!=")]
    NotEqual,

    #[token("==")]
    Equal,

    #[token("=")]
    Assignment,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("<=")]
    LessThanEqual,

    #[token(">=")]
    GreaterThanEqual,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(",")]
    Comma,

    #[token(";")]
    SemiColon,

}
