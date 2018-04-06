use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Def,
    Extern,
    Ident(String),
    Number(f32),
    Kwd(char),
}

fn match_number(str: &[u8]) -> (usize, Option<Token>) {
    let num_end_pos = str.iter().position(|v| !(*v as char).is_digit(10));

    (
        num_end_pos.unwrap_or(str.len()),
        num_end_pos
            .map(|i| Token::Number(FromStr::from_str(str::from_utf8(&str[..i]).unwrap()).unwrap())),
    )
}

fn match_number_or_kwd(str: &[u8]) -> (usize, Option<Token>) {
    match str[0] as char {
        '\0' => (1, None),
        '#' => (str.iter().position(|v| *v == b'\n').unwrap_or(str.len()), None),
        c if c.is_digit(10) => match_number(&str),
        c => (1, Some(Token::Kwd(c))),
    }
}

fn first_match(str: &[u8]) -> (usize, Option<Token>) {
    let ws = str.iter()
        .take_while(|v| (**v as char).is_whitespace())
        .count();

    if ws >= str.len() {
        return (ws, None);
    }

    let first = str.iter()
        .skip(ws)
        .position(|v| !(*v as char).is_alphabetic());

    match first {
        Some(0) => {
            let res = match_number_or_kwd(&str[ws..]);
            (res.0 + ws, res.1)
        }
        Some(i) => (
            ws + first.unwrap_or(str.len()),
            Some(match str[ws..ws + i].as_ref() {
                b"def" => Token::Def,
                b"extern" => Token::Extern,
                a => Token::Ident(str::from_utf8(a).unwrap().to_string()),
            }),
        ),
        None => (ws + first.unwrap_or(str.len()), None),
    }
}

pub fn lex(str: &[u8]) -> Vec<Token> {
    let mut all = vec![];
    let mut left = str;

    while !left.is_empty() {
        let res = first_match(left);
        left = &left[res.0..];
        let tok = res.1;
        tok.map(|t| all.push(t));
    }

    all
}
