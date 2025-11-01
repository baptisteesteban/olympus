// Small experimental binary to parse an OFF file with nom (https://shape.cs.princeton.edu/benchmark/documentation/off_format.html)

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::newline;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

fn parse_header(input: &str) -> IResult<&str, Vec<usize>> {
    let mut magic = tag::<&str, &str, Error<_>>("OFF\n");
    let is_digit = |c| c as u8 >= b'0' && c as u8 <= b'9';
    let mut alpha = separated_list1(tag::<&str, &str, Error<_>>(" "), take_while(is_digit));
    let (input, _) = magic.parse(input).unwrap();
    let (remain, num_elements) = alpha.parse(input).unwrap();
    let vec_size = num_elements
        .iter()
        .map(|e| str::parse(e).unwrap())
        .collect::<Vec<usize>>();
    let (remain, _) = newline::<&str, Error<_>>(remain).unwrap();
    Ok((remain, vec_size))
}

fn parse_off(content: &str) -> IResult<&str, Vec<usize>> {
    parse_header(content)
}

fn main() {
    let (remaining, output) =
        parse_off("OFF\n3 1 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n").unwrap();
    println!("Remaining:\n\"\"\"\n{}\"\"\"", remaining);
    println!("Remaining size: {}", remaining.len());
    output.iter().for_each(|e| println!("{}", e));
}
