mod lexer;
mod parser;

fn main() {
    let str = b"def fib(x)
#hej
if x < 3 then
1
else
fib(x-1)+fib(x-2)";
    let res = lexer::lex(str);

    println!("{:?}", res);
}
