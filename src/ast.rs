//! An AST to represent Verilog code. Useful for programmatically emitting Verilog.
#![allow(dead_code)]

/// A Verilog identifier. Could be a port, wire, register, etc.
#[derive(Debug)]
pub struct Ident(pub String);

impl Ident {
    pub fn new<S>(ident: S) -> Ident
    where
        S: Into<String>,
    {
        Ident(ident.into())
    }
}

/// Port directionality.
#[derive(Debug)]
pub enum Dir {
    In,
    Out,
}

/// Edge side (rising/+ or falling/-).
#[derive(Debug)]
pub enum Edge {
    Rising,
    Falling,
}

/// A trigger (edge side + signal).
#[derive(Debug)]
pub struct Trigger {
    /// Identifier of the signal to trigger on
    pub signal: Ident,

    /// Signal transition edge to trigger on
    pub edge: Edge,
}

/// Operators with arity two.
#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,

    Or,
    And,

    BitOr,
    BitAnd,
    LShift,
    RShift,
}

/// Operators with arity one.
#[derive(Debug)]
pub enum UnaryOp {
    Not,
}
