// Small experimental binary to parse an OFF file with nom (https://shape.cs.princeton.edu/benchmark/documentation/off_format.html)

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::error::Error;
use nom::number::complete::float;
use nom::{IResult, Parser};

use crate::{Mesh, Point3d, Triangle};

fn parse_number(input: &str) -> IResult<&str, usize> {
    let (remain, v_str) = digit1::<&str, Error<_>>(input).unwrap();
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

fn parse_vertices(input: &str, n_vertices: usize) -> IResult<&str, Vec<Point3d>> {
    let mut v = Vec::with_capacity(n_vertices);

    let (mut x, mut y, mut z): (f32, f32, f32);
    let mut remaining = input;
    for _ in 0..n_vertices {
        (remaining, (x, _, y, _, z, _)) = (
            float::<&str, Error<_>>,
            space1,
            float,
            space1,
            float,
            newline,
        )
            .parse(remaining)
            .unwrap();
        v.push(Point3d::new(x, y, z));
    }

    Ok((remaining, v))
}

fn parse_triangles(input: &str, n_triangles: usize) -> IResult<&str, Vec<Triangle>> {
    let mut v = Vec::with_capacity(n_triangles);

    let mut remaining = input;
    let (mut v1, mut v2, mut v3): (usize, usize, usize);
    for _ in 0..n_triangles {
        /*let n_vert: usize;*/
        (remaining, (/*n_vert*/ _, _)) = (parse_number, space1).parse(remaining).unwrap();
        /*if n_vert != 3 {
            return Err(Error);
        }*/
        (remaining, (v1, _, v2, _, v3, _)) = (
            parse_number,
            space1,
            parse_number,
            space1,
            parse_number,
            newline,
        )
            .parse(remaining)
            .unwrap();
        v.push(Triangle::new(v1, v2, v3));
    }

    Ok((remaining, v))
}

pub fn parse_off(content: &str) -> IResult<&str, Mesh> {
    let (content, (n_vertices, n_triangles, _)) = parse_header(content).unwrap();
    let (content, vertices) = parse_vertices(content, n_vertices).unwrap();
    let (content, triangles) = parse_triangles(content, n_triangles).unwrap();
    Ok((content, Mesh::new(vertices, triangles)))
}
