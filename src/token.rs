#[derive(Debug, Clone, Eq, PartialEq)]
#[rustfmt::skip]
pub enum Token {
    Identifier(String),

    // ============== //
    //  declarations  //
    // ============== //

    Let,
    Fn,

    // ============== //
    //  control flow  //
    // ============== //

    If,
    Else,
    Match,
    Loop,
    While,
    Break,
    Continue,
    Return,

    // =========== //
    //  operators  //
    // =========== //

    Assign,
    Call,

    Add,
    AddAndAsign,
    Subtract,
    SubtractAndAssign,
    Multiply,
    MultiplyAndAssign,
    Divide,
    DivideAndAssign,

    Equal,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,


    // ============ //
    //  delimiters  //
    // ============ //

    Dot,
    Comma,
    Semicolon,

    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    LeftAngleBracket,
    RightAngleBracket,
}
