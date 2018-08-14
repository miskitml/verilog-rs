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
#[derive(Debug, Display)]
pub enum Dir {
    #[strum(to_string="in")]
    In,
    #[strum(to_string="out")]
    Out,
}

/// Edge side (rising/+ or falling/-).
#[derive(Debug, Display)]
pub enum Edge {
    #[strum(to_string="posedge")]
    Rising,
    #[strum(to_string="negedge")]
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
#[derive(Debug, Display)]
pub enum BinaryOp {
    #[strum(to_string="+")]
    Add,
    #[strum(to_string="-")]
    Sub,
    #[strum(to_string="*")]
    Mul,
    #[strum(to_string="/")]
    Div,

    #[strum(to_string="==")]
    Eq,
    #[strum(to_string="!=")]
    Neq,
    #[strum(to_string="<")]
    Lt,
    #[strum(to_string=">")]
    Gt,
    #[strum(to_string="<=")]
    Lte,
    #[strum(to_string=">=")]
    Gte,

    #[strum(to_string="||")]
    Or,
    #[strum(to_string="&&")]
    And,

    #[strum(to_string="|")]
    BitOr,
    #[strum(to_string="&")]
    BitAnd,
    #[strum(to_string="<<")]
    LShift,
    #[strum(to_string=">>")]
    RShift,
}

/// Operators with arity one.
#[derive(Debug, Display)]
pub enum UnaryOp {
    #[strum(to_string="!")]
    Not,
}
