use ast;
use std::fmt;

/// Implemented by AST nodes to emit Verilog.
pub trait Codegen {
    fn gen(&self) -> String;
}

/// All AST nodes that impl `Display` get a free `Codegen` impl.
impl<T: fmt::Display> Codegen for T {
    fn gen(&self) -> String {
        format!("{}", self)
    }
}

impl Codegen for ast::Trigger {
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
