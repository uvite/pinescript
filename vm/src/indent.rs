use nom::{
    self,
    character::complete::{alphanumeric1, line_ending, tab},
    combinator::{all_consuming, eof, map},
    multi::{many0, many_till},
    sequence::{pair, terminated},
};

type Resultnom<'a, T> = nom::IResult<&'a str, T, (&'a str, nom::error::ErrorKind)>;
use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
#[derive(Debug)]
enum Token {
    Dedent,
    Indent,
    Var(String),
}

#[derive(Debug)]
struct IndentationCounter {
    current: isize,
}

fn scan_lines<'a>(source: &'a str, counter: &mut IndentationCounter) -> Resultnom<'a, Vec<Token>> {
    let indent = |i| indentation(i, counter);
    let (rest, (tokens, _)) = all_consuming(many_till(
        terminated(
            map(
                pair(
                    indent,
                    map(alphanumeric1, |v: &str| Token::Var(v.to_string())),
                ),
                |(mut a, b)| {
                    a.push(b);
                    a
                },
            ),
            line_ending,
        ),
        eof,
    ))(source)?;
    Ok((rest, tokens.into_iter().flatten().collect()))
}

fn indentation<'a>(input: &'a str, counter: &mut IndentationCounter) -> Resultnom<'a, Vec<Token>> {
    let (rest, tabs) = many0(tab)(input)?;
    let mut indent_tokens = vec![];
    let indent_level = tabs.len() as isize;
    if indent_level < counter.current {
        for _ in 0..counter.current - indent_level {
            indent_tokens.push(Token::Dedent);
        }
    } else if indent_level > counter.current {
        for _ in 0..indent_level - counter.current {
            indent_tokens.push(Token::Indent);
        }
    }
    counter.current = indent_level;
    Ok((rest, indent_tokens))
}

fn scan(source: &str) -> Vec<Token> {
    let mut c = IndentationCounter { current: 0 };
    let (_, tokens) = scan_lines(&source, &mut c).expect("Failed to scan.");

    tokens
}

/*
    Var(
        "a",
    ),
    Indent,
    Var(
        "b",
    ),
    Indent,
    Var(
        "c",
    ),
    Dedent,
    Var(
        "d",
    ),
    Dedent,
    Var(
        "e",
    ),
]
 */
#[test]
fn maina() {
    //let source = "a\n\tb\n\t\tc\n\td\ne\n";
    let source = read_file("examples/test.mk".to_owned()).ok().unwrap();
    println!("{:?}",source);
    dbg!(scan(&source));
}