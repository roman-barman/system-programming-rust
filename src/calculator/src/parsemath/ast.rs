use std::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

pub fn eval(node: Node) -> Result<f64, Box<dyn error::Error>> {
    match node {
        Node::Number(n) => Ok(n),
        Node::Add(lhs, rhs) => Ok(eval(*lhs)? + eval(*rhs)?),
        Node::Subtract(lhs, rhs) => Ok(eval(*lhs)? - eval(*rhs)?),
        Node::Multiply(lhs, rhs) => Ok(eval(*lhs)? * eval(*rhs)?),
        Node::Divide(lhs, rhs) => Ok(eval(*lhs)? / eval(*rhs)?),
        Node::Caret(lhs, rhs) => Ok(eval(*lhs)?.powf(eval(*rhs)?)),
        Node::Negative(expr) => Ok(-eval(*expr)?),
    }
}