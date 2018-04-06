use lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(f32),
    Variable(String),
    Binary(char, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

type Prototype = (String, Vec<String>);

#[derive(Debug)]
pub enum Ast {
    Expr(Expr),
    Prototype(Prototype),
    Function(Prototype, Expr),
}

use std::borrow::BorrowMut;

fn parse_kwd<'a>(
    kwd: char,
    tokens: &[Token],
    stack: &mut Vec<Expr>,
) -> (usize, Result<Expr, String>) {
    let BINARY_OPERATORS = ['+', '-', '/', '*', '<', '>'];

    use self::Token::*;

    println!(">> it gun be guud parse {:?} {:?}", kwd, tokens.get(0));

    let lule = match kwd {
        '(' => match stack.pop() {
            Some(Expr::Variable(ref s)) => {
                let mut jmp = 0;
                let mut args = Vec::<Expr>::new();

                while tokens[jmp..].len() > 0 {
                    let t = &tokens[jmp];
                    println!("now da args is {:?}", args);
                    match t {
                        Def | Extern => panic!("wtf no def or extern in args"),
                        Kwd(',') => jmp += 1,
                        Kwd(')') => {
                            jmp += 1;
                            println!("yee boi");
                            break;
                        }
                        _ => match parse_expr(&tokens[jmp..], &mut args) {
                            (jmp2, Ok(e)) => {
                                args.push(e);
                                println!("after da args are {:?}", args);
                                jmp += jmp2;
                            }
                            (jmp2, Err(e)) => {
                                jmp += jmp2;
                                panic!(e);
                            }
                        },
                    }
                }

                (jmp, Ok(Expr::Call(s.clone(), args)))
            }

            _ => (1, Err("Nothing on the stack.".to_owned())),
        },
        ref c if BINARY_OPERATORS.contains(c) => match stack.pop() {
            Some(v) => {
                //let mut stack = Vec::<Expr>::new();

                let (jmp, expr) = parse_expr(&tokens[0..], stack);
                match expr {
                    Ok(e) => (jmp, Ok(Expr::Binary(*c, box v, box e))),
                    Err(e) => (jmp, Err(e)),
                }
            }

            _ => (1, Err("Nothing on the stack for binary.".to_owned())),
        },
        _ => (1, Ok(Expr::Number(1.0))),
    };

    println!("<< omfg {:?}", lule);

    lule
}

fn parse_expr<'a>(tokens: &[Token], stack: &'a mut Vec<Expr>) -> (usize, Result<Expr, String>) {
    use self::Expr::*;

    println!("gun parse {:?}", tokens.get(0));

    tokens
        .get(0)
        .map(|t| match t {
            Token::Number(n) => (1, Ok(Number(*n))),
            Token::Ident(ref s) => {
                let var = Variable(s.clone());

                match tokens.get(1) {
                    Some(Token::Kwd('(')) => {
                        stack.push(var);
                        let (jmp, res) = parse_expr(&tokens[1..], stack);
                        (jmp + 1, res)
                    }
                    _ => (1, Ok(var)),
                }
            }

            Token::Kwd(c) => {
                let (jmp, res) = parse_kwd(*c, &tokens[1..], stack);
                (jmp + 1, res)
            }
            _ => (1, Ok(Number(1.0))),
        })
        .unwrap_or((0, Err("Token-list is empty.".to_owned())))
}

enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

use self::OneOrMany::*;

fn parse_ast(tokens: &[Token]) -> (usize, Result<OneOrMany<Ast>, String>) {
    use self::Token::*;

    match tokens.get(0).ok_or("Token-list is empty.".to_owned()) {
        Err(e) => (0, Err(e)),
        Ok(t) => match t {
            Token::Def => match (tokens.get(1), tokens.get(2)) {
                (Some(Ident(ref name)), Some(Kwd('('))) => {
                    let mut args = vec![];
                    let mut jmp = 3;

                    while tokens[jmp..].len() > 0 {
                        let t = &tokens[jmp];
                        println!("now da args is {:?}", args);
                        match t {
                            Def | Extern => panic!("wtf no def or extern in args"),
                            Kwd(',') => jmp += 1,
                            Kwd(')') => {
                                jmp += 1;
                                println!("yee boi");
                                break;
                            }
                            Ident(ref arg) => {
                                jmp += 1;
                                args.push(arg.clone())
                            }

                            hm => return (2, Err(format!("{:?} is not , ) or Ident", hm))),
                        }
                    }

                    (jmp, Ok(One(Ast::Prototype((name.clone(), args)))))
                }
                _ => (
                    1,
                    Err("Def must be followed by Ident and Kwd('(')".to_owned()),
                ),
            },
            Token::Extern => (1, Err("Extern not implemented.".to_owned())),
            a => {
                let mut vec = vec![];
                let mut jmp = 0;

                while (jmp < tokens.len()) {
                    match parse_expr(&tokens[jmp..], &mut vec) {
                        (j, Err(e)) => {
                            jmp += j;
                            return (jmp, Err(e));
                        }
                        (j, Ok(e)) => {
                            jmp += j;
                            vec.push(e);
                        }
                    }
                }

                (
                    jmp,
                    Ok(Many(
                        vec.into_iter().map(|e| Ast::Expr(e)).collect::<Vec<_>>(),
                    )),
                )
            }
        },
    }
}

pub fn parse(tokens: &[Token]) -> Vec<Ast> {
    let mut ast: Vec<Ast> = vec![];

    let mut i = 0;

    while i < tokens.len() {
        let (jmp, node) = parse_ast(&tokens[i..]);
        match node {
            Ok(Many(mut a)) => ast.append(&mut a),
            Ok(One(a)) => ast.push(a),
            Err(e) => panic!(e),
        }
        i += jmp;
        println!("{:?}", i);
    }

    ast
}
