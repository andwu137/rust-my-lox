pub fn tokenize(source: String) -> Vec<Token> {
    vec![]
}

type Token = TokenType<TokenData>;

pub struct TokenData {
    source_column: u32,
    source_row: u32,
}

pub enum TokenType<A> {
    // Single-character tokens.
    LeftParen(A),
    RightParen(A),
    LeftBrace(A),
    RightBrace(A),
    Comma(A),
    Dot(A),
    Minus(A),
    Plus(A),
    Semicolon(A),
    Slash(A),
    Star(A),

    // One or two character tokens.
    Bang(A),
    BangEqual(A),
    Equal(A),
    EqualEqual(A),
    Greater(A),
    GreaterEqual(A),
    Less(A),
    LessEqual(A),

    // Literals.
    Identifier(A, String),
    String(A, String),
    Number(A, String),

    // Keywords.
    And(A),
    Class(A),
    Else(A),
    False(A),
    Fun(A),
    For(A),
    If(A),
    Nil(A),
    Or(A),
    Print(A),
    Return(A),
    Super(A),
    This(A),
    True(A),
    Var(A),
    While(A),

    Eof(A),
}
