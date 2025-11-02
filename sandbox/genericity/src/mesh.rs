use std::fmt::Display;

pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3d {
    pub fn new(x: f32, y: f32, z: f32) -> Point3d {
        Point3d { x, y, z }
    }
}

impl Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {}, {})", self.x, self.y, self.z).as_str())
    }
}

pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
}

impl Triangle {
    pub fn new(v1: usize, v2: usize, v3: usize) -> Triangle {
        Triangle { v1, v2, v3 }
    }
}

#[derive(Default)]
pub struct Mesh {
    pos: Vec<Point3d>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(vertices: Vec<Point3d>, triangles: Vec<Triangle>) -> Mesh {
        Mesh {
            pos: vertices,
            triangles,
        }
    }

    #[inline]
    pub fn vertices(&self) -> impl Iterator<Item = &Point3d> {
        self.pos.iter()
    }

    #[inline]
    pub fn triangles(&self) -> impl Iterator<Item = &Triangle> {
        self.triangles.iter()
    }

    #[inline]
    pub fn num_vertices(&self) -> usize {
        self.pos.len()
    }

    #[inline]
    pub fn num_triangles(&self) -> usize {
        self.triangles.len()
    }
}
