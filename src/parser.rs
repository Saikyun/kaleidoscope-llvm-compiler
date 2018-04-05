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

fn parse_expr(tokens: &[Token], ast: &mut Vec<Ast>) -> (usize, Option<Expr>) {
    match &tokens[0] {
        Token::Kwd(c) => parse_kwd(&tokens, ast),
        Token::Number(f) => (1, Some(Expr::Number(*f))),
        Token::Ident(ref s) => parse_ident(&tokens, ast),
        _ => (1, None),
    }
}

fn parse_binop(op: &Token, left_side: Expr, tokens: &[Token], ast: &mut Vec<Ast>) -> (usize, Option<Expr>) {
    let (jmp, right_side) = parse_primary_expr(&tokens, ast);

    let right_side = match right_side {
        Some(e) => e,
        _ => panic!(format!("{:?} is not an Expr", right_side)),
    };

    match op {
        Token::Kwd(c) => (
            1 + jmp,
            Some(Expr::Binary(*c, box left_side, box right_side)),
        ),
        _ => panic!(format!("{:?} is not Kwd operator", op)),
    }
}

fn parse_prototype(tokens: &[Token]) -> (usize, Option<Prototype>) {
    let name = &tokens[0];

    let name = match name {
        Token::Ident(s) => s,
        _ => panic!(format!("{:?} is not an ident", name)),
    };

    let args = tokens[2..]
        .iter()
        .take_while(|v| match v {
            Token::Kwd(')') => false,
            _ => true,
        })
        .filter_map(|v| match v {
            Token::Kwd(',') => None,
            Token::Ident(s) => Some(s.clone()),
            _ => panic!(format!("{:?} is not , or Ident", v)),
        })
        .collect::<Vec<String>>();

    (1 + 2 + (args.len() * 2 - 1), Some((name.clone(), args)))
}

fn parse_ast(tokens: &[Token], ast: &mut Vec<Ast>) -> usize {
    match tokens[0] {
        Token::Def => {
            let (jmp, proto) = parse_prototype(&tokens[1..]);
            proto.map(|v| ast.push(Ast::Prototype(v)));
            jmp + 1
        }
        ref a => {
            let (jmp, expr) = parse_primary_expr(&tokens, ast);
            expr.map(|v| ast.push(Ast::Expr(v)));
            jmp
        }
    }
}

fn parse_primary_expr(tokens: &[Token], ast: &mut Vec<Ast>) -> (usize, Option<Expr>) {
    let operators = ['+', '-', '<'];

    println!("{:?}", tokens);
    let (jmp, expr) = match tokens[0] {
        ref e => parse_expr(&tokens, ast),
    };

    if expr.is_none() { return (jmp, None); }

    let expr = expr.unwrap();

    match &tokens.get(jmp) {
        Some(Token::Kwd(c)) if operators.contains(c) => {
            /*let last_expr = match ast.pop() {
            Some(Ast::Expr(e)) => e,
            a => panic!(format!("{:?} is not Expr", a)),
        };*/

            println!("consumed {:?}", expr);
            let (jmp2, expr2) = parse_binop(&tokens[jmp], expr, &tokens[jmp + 1..], ast);

                        println!("{:?}", expr2);
            println!("jump {:?}", jmp + jmp2);

            (jmp + jmp2, expr2)
        }
        _ => (jmp, Some(expr)),
    }
}

fn parse_args(tokens: &[Token], ast: &mut Vec<Ast>) -> (usize, Vec<Expr>) {
    match tokens[0] {
        Token::Kwd(')') => (0, vec![]),
        _ => {
            let (jmp1, first) = parse_primary_expr(&tokens[0..], ast);

            let (jmp2, mut more) = match tokens[jmp1] {
                Token::Kwd(',') => parse_args(&tokens[jmp1 + 1..], ast),
                Token::Kwd(')') => (0, vec![]),
                _ => (1, vec![]),
            ref t => panic!(format!("{:?} is not , or )", t)),
    };

    first.map(|v| more.push(v));
    (jmp1 + jmp2, more)
}
    }
}

fn parse_ident(tokens: &[Token], ast: &mut Vec<Ast>) -> (usize, Option<Expr>) {
    match tokens[0] {
        Token::Ident(ref s) => {
            let next = tokens.get(1);
            match next {
                Some(Token::Kwd('(')) => {
                    let (jmp, args) = parse_args(&tokens[2..], ast);
                    let closing = &tokens[jmp + 1];
                    (jmp + 3, Some(Expr::Call(s.clone(), args)))
                }
                _ => (1, Some(Expr::Variable(s.clone()))),
            }
        }
        _ => (1, None),
        ref t => panic!(format!("{:?} is not Ident", t)),
    }
}

fn parse_kwd(tokens: &[Token], ast: &mut Vec<Ast>) -> (usize, Option<Expr>) {
    match tokens[0] {
        Token::Kwd('(') => {
            let (jmp, expr) = parse_expr(&tokens[1..], ast);
            let closing = &tokens[jmp + 1];
            (jmp + 3, expr)
        }
        _ => (1, None),
        ref t => panic!(format!("{:?} is not Kwd", t)),
    }
}

pub fn parse(tokens: &[Token]) -> Vec<Ast> {
    let mut ast: Vec<Ast> = vec![];

    let mut i = 0;

    while i < tokens.len() {
        let jmp = parse_ast(&tokens[i..], &mut ast);
        i += jmp;
    }

    ast
}
