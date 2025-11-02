// Small experimental binary to parse an OFF file with nom (https://shape.cs.princeton.edu/benchmark/documentation/off_format.html)

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{newline, space1};
use nom::error::Error;
use nom::{IResult, Parser};

fn parse_number(input: &str) -> IResult<&str, usize> {
    let is_digit = |v: char| v as u8 >= b'0' && v as u8 <= b'9';
    let (remain, v_str) = take_while1::<_, &str, Error<_>>(is_digit)
        .parse(input)
        .unwrap();
    let v = v_str.parse::<usize>().unwrap();
    Ok((remain, v))
}

fn parse_header(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag::<&str, &str, Error<_>>("OFF\n").parse(input).unwrap();
    let (remain, (n_vertices, _, n_faces, _, n_edges, _)) = (
        parse_number,
        space1,
        parse_number,
        space1,
        parse_number,
        newline,
    )
        .parse(input)
        .unwrap();
    Ok((remain, (n_vertices, n_faces, n_edges)))
}

fn parse_off(content: &str) -> IResult<&str, (usize, usize, usize)> {
    parse_header(content)
}

fn main() {
    let (remaining, (n_vertices, n_faces, n_edges)) =
        parse_off("OFF\n3 1 0\n0.0 0.0 0.0\n1.0 0.0 0.0\n0.5 1.0 0.0\n3 0 1 2\n").unwrap();
    println!(
        "Vertices: {} Faces: {} Edges: {}",
        n_vertices, n_faces, n_edges
    );
    println!("Remaining:\n\"\"\"\n{}\"\"\"", remaining);
    println!("Remaining size: {}", remaining.len());
}
