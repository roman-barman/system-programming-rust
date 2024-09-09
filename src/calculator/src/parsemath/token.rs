pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        match *self {
            Token::Add | Token::Subtract => OperPrec::AddSub,
            Token::Multiply | Token::Divide => OperPrec::MulDiv,
            Token::Caret => OperPrec::Power,
            _ => OperPrec::DefaultZero,
        }
    }
}