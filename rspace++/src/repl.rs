
use std::io::{self, Write};
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, digit1, multispace0, multispace1}, combinator::{map, recognize}, multi::many0, sequence::{preceded, terminated}};
use nom::multi::separated_list0;

// Define the RChain tutorial grammar
fn parse_space(input: &str) -> IResult<&str, &str> {
    alt((
        tag("Nil"),
        recognize(preceded(tag("Cons("), terminated(parse_list, tag(")")))),
        recognize(preceded(tag("("), terminated(parse_list, tag(")")))),
        recognize(preceded(alpha1, many0(terminated(alt((alpha1, digit1)), multispace0))))
    ))(input)
}

fn parse_list(input: &str) -> IResult<&str, &str> {
    map(separated_list0(tag(","), parse_space), |_| "")(input)
}

fn parse_input(input: &str) -> IResult<&str, &str> {
    alt((
        tag("quit"),
        parse_space
    ))(input)
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match parse_input(input.trim()) {
            Ok((_, "quit")) => break,
            Ok((_, result)) => println!("{}", result),
            Err(e) => println!("Error: {:?}", e)
        }
    }
}
