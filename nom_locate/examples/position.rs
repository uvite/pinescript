extern crate nom;
extern crate nom_locate;

use nom::bytes::complete::{tag, take_until};
use nom::IResult;
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

struct Token<'a> {
    pub position: Span<'a>,
    pub foo: &'a str,
    pub bar: &'a str,
}

fn parse_foobar(s: Span) -> IResult<Span, Token> {
    let (s, _) = take_until("foo")(s)?;
    println!("{:?}",s.get);
    let (s, pos) = position(s)?;
    println!("{:?}-{:?}",s,pos);

    let (s, foo) = tag("foo")(s)?;
    println!("{:?}",s);
    let (s, bar) = tag("bar")(s)?;
    println!("{:?}",s);
    Ok((
        s,
        Token {
            position: pos,
            foo: foo.fragment(),
            bar: bar.fragment(),
        },
    ))
}

fn main() {
    let input = Span::new("Lorem ipsum \n ok foobar24324");
    let output = parse_foobar(input);
    let position = output.unwrap().1.position;
    println!("{:?}",position);

    assert_eq!(position, unsafe {
        Span::new_from_raw_offset(
            14, // offset
            2,  // line
            "", // fragment
            (), // extra
        )
    });
    assert_eq!(position.get_column(), 2);
}
