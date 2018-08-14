use ast::{BinaryOp, Dir, Edge, Ident, Trigger, UnaryOp};
use std::fmt;

/// Implemented by AST nodes to emit Verilog.
pub trait Codegen {
    fn gen(&self) -> String;
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match *self {
            Edge::Rising => "posedge",
            Edge::Falling => "negedge",
        };

        write!(f, "{}", v)
    }
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

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UnaryOp::*;
        let v = match *self {
            Not => "!",
        };

        write!(f, "{}", v)
    }
}

/// All AST nodes that impl `Display` get a free `Codegen` impl.
impl<T: fmt::Display> Codegen for T {
    fn gen(&self) -> String {
        format!("{}", self)
    }
}

impl Codegen for Trigger {
    fn gen(&self) -> String {
        format!("{} {}", self.edge, self.signal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::*;

    #[test]
    fn triggers() {
        let rising = Trigger {
            signal: Ident::new("foo"),
            edge: Edge::Rising,
        };

        assert_eq!(rising.gen(), "posedge foo");

        let falling = Trigger {
            signal: Ident::new("bar"),
            edge: Edge::Falling,
        };

        assert_eq!(falling.gen(), "negedge bar");
    }
}
