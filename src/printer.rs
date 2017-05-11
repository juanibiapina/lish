use ast::LispExpr;

pub struct Printer;

impl Printer {
    pub fn new() -> Printer {
        Printer
    }

    pub fn print(&self, lisp_expr: &LispExpr) -> String {
        match *lisp_expr {
            LispExpr::Symbol(ref s) => s.clone(),
            LispExpr::List(ref exprs) => self.print_list(exprs),
        }
    }

    fn print_list(&self, exprs: &Vec<LispExpr>) -> String {
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
