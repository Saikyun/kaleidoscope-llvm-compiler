use lexer::Token;
use std::collections::HashMap;

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

fn parse_kwd<'a>(
    kwd: char,
    tokens: &[Token],
    stack: &mut Vec<Expr>,
) -> (usize, Result<Expr, String>) {
    let binary_operators = ['+', '-', '/', '*', '<', '>'];
    let mut precedence = HashMap::new();
    precedence.insert('/', 10);
    precedence.insert('+', 20);
    precedence.insert('-', 20);
    precedence.insert('*', 40);

    use self::Token::*;

    let lule = match kwd {
        '(' => match stack.pop() {
            Some(Expr::Variable(ref s)) => {
                let mut jmp = 0;
                let mut args = Vec::<Expr>::new();

                while tokens[jmp..].len() > 0 {
                    let t = &tokens[jmp];
                    match t {
                        Def | Extern => panic!("wtf no def or extern in args"),
                        Kwd(',') => jmp += 1,
                        Kwd(')') => {
                            jmp += 1;
                            break;
                        }
                        _ => match parse_expr(&tokens[jmp..], &mut args) {
                            (jmp2, Ok(e)) => {
                                args.push(e);
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

            _ => {
                let mut jmp = 0;
                let mut stack = Vec::<Expr>::new();

                while tokens[jmp..].len() > 0 {
                    let t = &tokens[jmp];
                    match t {
                        Def | Extern => panic!("wtf no def or extern in args"),
                        Kwd(',') => jmp += 1,
                        Kwd(')') => {
                            jmp += 1;
                            break;
                        }
                        _ => match parse_expr(&tokens[jmp..], &mut stack) {
                            (jmp2, Ok(e)) => {
                                stack.push(e);
                                jmp += jmp2;
                            }
                            (jmp2, Err(e)) => {
                                jmp += jmp2;
                                panic!(e);
                            }
                        },
                    }
                }

                (jmp, Ok(stack.remove(0)))
            }
        },
        ref c if binary_operators.contains(c) => match stack.pop() {
            Some(Expr::Binary(c2, e1, e2)) => {
                let (jmp, rhs) = parse_expr(&tokens[0..], stack);

                let rhs = match rhs {
                    Ok(expr) => expr,
                    Err(e) => return (jmp, Err(e)),
                };

                let bin = match (precedence.get(&c), precedence.get(&c2)) {
                    (Some(i1), Some(i2)) => {
                        if i2 >= i1 {
                            let lhs = Expr::Binary(c2, e1, e2);
                            Ok(Expr::Binary(*c, box lhs, box rhs))
                        } else {
                            let lhs = e1;
                            let rhs = Expr::Binary(*c, e2, box rhs);
                            Ok(Expr::Binary(c2, lhs, box rhs))
                        }
                    }
                    _ => Err(format!("No precedence for {:?} or {:?}", c, c2)),
                };

                (jmp, bin)
            }
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

    lule
}

fn parse_expr<'a>(tokens: &[Token], stack: &'a mut Vec<Expr>) -> (usize, Result<Expr, String>) {
    use self::Expr::*;

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
                        match t {
                            Def | Extern => panic!("wtf no def or extern in args"),
                            Kwd(',') => jmp += 1,
                            Kwd(')') => {
                                jmp += 1;
                                break;
                            }
                            Ident(ref arg) => {
                                jmp += 1;
                                args.push(arg.clone())
                            }

                            hm => return (2, Err(format!("{:?} is not , ) or Ident", hm))),
                        }
                    }

                    let proto = (name.clone(), args);

                    let (jmp2, body) = parse_expr_list(&tokens[jmp..]);
                    let mut body = match body {
                        Ok(body) => body,
                        Err(e) => return (jmp + jmp2, Err(e)),
                    };

                    (jmp + jmp2, Ok(One(Ast::Function(proto, body.remove(0)))))
                }
                _ => (
                    1,
                    Err("Def must be followed by Ident and Kwd('(')".to_owned()),
                ),
            },
            Token::Extern => (1, Err("Extern not implemented.".to_owned())),
            a => {
                let (jmp, body) = parse_expr_list(&tokens[0..]);
                (
                    jmp,
                    body.map(|vec| Many(vec.into_iter().map(|e| Ast::Expr(e)).collect::<Vec<_>>())),
                )
            }
        },
    }
}

pub fn parse_expr_list(tokens: &[Token]) -> (usize, Result<Vec<Expr>, String>) {
    let mut vec = vec![];
    let mut jmp = 0;

    while jmp < tokens.len() {
        match tokens[jmp] {
            Token::Kwd(';') => {
                jmp += 1;
                break;
            }
            _ => (),
        }

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

    (jmp, Ok(vec))
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
        //println!("{:?}", i);
    }

    ast
}
