//! An AST to represent Verilog code. Useful for programmatically emitting Verilog.
#![allow(dead_code)]

use std::fmt;

/// A Verilog identifier. Could be a port, wire, register, etc.
#[derive(Debug)]
pub struct Ident(String);

impl Ident {
    pub fn new<S>(ident: S) -> Ident
    where
        S: Into<String>,
    {
        Ident(ident.into())
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Port directionality.
#[derive(Debug)]
pub enum Dir {
    In,
    Out,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match *self {
            Dir::In => "in",
            Dir::Out => "out",
        };

        write!(f, "{}", v)
    }
}

/// Edge side (rising/+ or falling/-).
#[derive(Debug)]
pub enum Edge {
    Rising,
    Falling,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match *self {
            Edge::Rising => "posedge",
            Edge::Falling => "negedge",
        };

        write!(f, "{}", v)
    }
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

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BinaryOp::*;
        let v = match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",

            Eq => "==",
            Neq => "!=",
            Lt => "<",
            Gt => ">",
            Lte => "<=",
            Gte => ">=",

            Or => "||",
            And => "&&",

            BitOr => "|",
            BitAnd => "&",
            LShift => "<<",
            RShift => ">>",
        };

        write!(f, "{}", v)
    }
}


/// Operators with arity one.
#[derive(Debug)]
pub enum UnaryOp {
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UnaryOp::*;
        let v = match *self {
            Not => "!",
        };

        write!(f, "{}", v)
    }
}
