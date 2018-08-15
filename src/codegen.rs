use ast::{Assignment, BinaryOp, Expression, Ident, Literal, Trigger};
use std::fmt::{self, Write};

/// Implemented by AST nodes to emit Verilog.
pub trait Codegen {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write;

    fn to_string(&self) -> String {
        let mut string = String::new();
        self.gen(&mut string).unwrap();
        string
    }
}

/// All AST nodes that impl `Display` get a free `Codegen` impl.
impl<T: fmt::Display> Codegen for T {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        write!(w, "{}", self)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Codegen for Literal {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        match self {
            Literal::Integer(x) => write!(w, "{}", x),
        }
    }
}

impl Codegen for Trigger {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        write!(w, "{} {}", self.edge, self.signal)
    }
}

impl Codegen for Expression {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        match self {
            Expression::Ident(id) => id.gen(w),
            Expression::Literal(lit) => lit.gen(w),
            Expression::BinaryOp(lit) => {
                write!(w, "(")?;
                lit.gen(w)?;
                write!(w, ")")
            }
        }
    }
}

impl Codegen for Assignment {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        write!(w, "assign {lhs} = ", lhs = self.lhs)?;
        self.rhs.gen(w)?;
        write!(w, ";")
    }
}

impl Codegen for BinaryOp {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        self.lhs.gen(w)?;
        write!(w, " {} ", self.ty)?;
        self.rhs.gen(w)
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

        assert_eq!(rising.to_string(), "posedge foo");

        let falling = Trigger {
            signal: Ident::new("bar"),
            edge: Edge::Falling,
        };

        assert_eq!(falling.to_string(), "negedge bar");
    }

    #[test]
    fn assignments() {
        let assign = Assignment {
            lhs: Ident::new("foo"),
            rhs: Expression::Ident(Ident::new("bar")),
        };

        assert_eq!(assign.to_string(), "assign foo = bar;");
    }

    #[test]
    fn binary_op() {
        let add = BinaryOp {
            ty: BinaryOpTy::Add,
            lhs: Expression::Ident(Ident::new("a")),
            rhs: Expression::Ident(Ident::new("b")),
        };

        assert_eq!(add.to_string(), "a + b");
    }

    #[test]
    fn literal() {
        let int = Literal::Integer(1612);
        assert_eq!(int.to_string(), "1612");
    }
}
