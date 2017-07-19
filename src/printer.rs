use types::{LispType, LispValue};

pub struct Printer;

impl Printer {
    pub fn new() -> Printer {
        Printer
    }

    pub fn print(&self, lisp_expr: &LispValue, readable: bool) -> String {
        match **lisp_expr {
            LispType::Nil => "nil".to_owned(),
            LispType::Integer(i) => i.to_string(),
            LispType::Strn(ref s) => {
                if readable {
                    format!("\"{}\"", s.clone())
                } else {
                    s.clone()
                }
            },
            LispType::Symbol(ref s) => s.clone(),
            LispType::Function(_) => "#<function ...>".to_owned(),
            LispType::NativeFunction(_) => "#<native-function ...>".to_owned(),
            LispType::List(ref exprs) => self.print_list(exprs, readable),
        }
    }

    fn print_list(&self, exprs: &Vec<LispValue>, readable: bool) -> String {
        let mut first = true;
        let mut res = String::new();

        res.push_str("(");

        for expr in exprs.iter() {
            if first {
                first = false;
            } else {
                res.push_str(" ");
            }
            res.push_str(&self.print(expr, readable));
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
    use env;

    fn mock_func(_: &[LispValue]) -> Result<LispValue> {
        Ok(types::integer(3))
    }

    fn print(lisp_expr: &LispValue) -> String {
        Printer::new().print(lisp_expr, true)
    }

    fn display(lisp_expr: &LispValue) -> String {
        Printer::new().print(lisp_expr, false)
    }

    #[test]
    fn print_nil() {
        assert_eq!(print(&types::nil()), "nil");
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
    fn print_string() {
        assert_eq!(print(&types::string("lol".to_owned())), "\"lol\"");
    }

    #[test]
    fn print_string_in_list() {
        assert_eq!(
            print(
                &types::list(
                    vec![
                        types::string("lol".to_owned()),
                        types::string("lol2".to_owned())
                    ]
                )
            ),
            "(\"lol\" \"lol2\")"
        );
    }

    #[test]
    fn display_integer() {
        assert_eq!(display(&types::integer(1)), "1");
    }

    #[test]
    fn display_string() {
        assert_eq!(display(&types::string("lol".to_owned())), "lol");
    }

    #[test]
    fn display_string_in_list() {
        assert_eq!(
            display(
                &types::list(
                    vec![
                        types::string("lol".to_owned()),
                        types::string("lol2".to_owned())
                    ]
                )
            ),
            "(lol lol2)"
        );
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
    fn print_function() {
        assert_eq!(
            print(&types::function(vec!["a".to_owned()], types::integer(2), env::env_new(None))),
            "#<function ...>"
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
