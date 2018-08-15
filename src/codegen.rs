use ast::{Ident, Trigger};
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

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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

impl Codegen for Trigger {
    fn gen<W>(&self, w: &mut W) -> fmt::Result
    where
        W: Write,
    {
        write!(w, "{} {}", self.edge, self.signal)
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
}
