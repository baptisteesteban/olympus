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

pub struct Triangle {
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
}

impl Triangle {
    pub fn new(v1: i32, v2: i32, v3: i32) -> Triangle {
        Triangle { v1, v2, v3 }
    }
}

#[derive(Default)]
pub struct Mesh {
    pos: Vec<Point3d>,
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn with_capacity(n_vertices: i32, n_faces: i32) -> Mesh {
        Mesh {
            pos: Vec::<Point3d>::with_capacity(n_vertices as usize),
            triangles: Vec::<Triangle>::with_capacity(n_faces as usize),
        }
    }

    #[inline]
    pub fn add_vertex(&mut self, p: Point3d) {
        self.pos.push(p);
    }

    #[inline]
    pub fn add_triangle(&mut self, t: Triangle) {
        self.triangles.push(t);
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
