#[derive(Debug)]
pub enum Expr {
    Number(f32),
    Variable(String),
    Binary(char, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>)
}

type Prototype = (String, Vec<String>);
type Function = (Prototype, Expr);

fn main() {

}
