use types::{LispType, LispValue};

pub struct Printer;

impl Printer {
    pub fn new() -> Printer {
        Printer
    }

    pub fn print(&self, lisp_expr: &LispValue) -> String {
        match **lisp_expr {
            LispType::Integer(i) => i.to_string(),
            LispType::Symbol(ref s) => s.clone(),
            LispType::NativeFunction(_) => "#<native-function ...>".to_owned(),
            LispType::List(ref exprs) => self.print_list(exprs),
        }
    }

    fn print_list(&self, exprs: &Vec<LispValue>) -> String {
        let mut first = true;
        let mut res = String::new();

        res.push_str("(");

        for expr in exprs.iter() {
            if first {
                first = false;
            } else {
                res.push_str(" ");
            }
            res.push_str(&self.print(expr));
        }
        res.push_str(")");
        res
    }
}

#[cfg(test)]
mod tests {
    use types;
    use super::*;
    use error::Result;

    fn mock_func(_: &[LispValue]) -> Result<LispValue> {
        Ok(types::integer(3))
    }

    fn print(lisp_expr: &LispValue) -> String {
        Printer::new().print(lisp_expr)
    }

    #[test]
    fn print_integer() {
        assert_eq!(print(&types::integer(-34)), "-34");
    }

    #[test]
    fn print_symbol() {
        assert_eq!(print(&types::symbol("lol".to_owned())), "lol");
    }

    #[test]
    fn print_list() {
        assert_eq!(
            print(
                &types::list(
                    vec![
                        types::symbol("lol".to_owned()),
                        types::symbol("lol2".to_owned())
                    ]
                )
            ),
            "(lol lol2)"
        );
    }

    #[test]
    fn print_native_function() {
        assert_eq!(
            print( &types::native_function(mock_func)),
            "#<native-function ...>"
        );
    }

    #[test]
    fn print_nested_list() {
        assert_eq!(
            print(
                &types::list(
                    vec![
                        types::list(
                            vec![
                                types::symbol("lol".to_owned())
                            ]
                        ),
                        types::symbol("lol2".to_owned())
                    ]
                )
            ),
            "((lol) lol2)"
        );
    }
}
