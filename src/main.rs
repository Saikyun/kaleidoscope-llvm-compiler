#![feature(box_syntax)]

mod lexer;
mod parser;

fn main() {
    let str = b"def fib(x)
#hej
if x < 3 then
1
else
fib(x-1+3)+fib(x-2)+fib(x-3)\0";
    let str = b"def fib(x) fib(x-99+3); 23+2+fib(x-4)\0";

    let res = lexer::lex(str);
    println!("{:?}", res);
    let expr = parser::parse(&res);
    println!("{:?}", expr);
}
