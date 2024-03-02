use olympus::Point2d;

pub fn is_2_face(p: &Point2d) -> bool {
    p.x() % 2 == 0 && p.y() % 2 == 0
}

pub fn is_1h_face(p: &Point2d) -> bool {
    p.x() % 2 == 1 && p.y() % 2 == 0
}

pub fn is_1v_face(p: &Point2d) -> bool {
    p.x() % 2 == 0 && p.y() % 2 == 1
}

pub fn is_1_face(p: &Point2d) -> bool {
    is_1h_face(p) || is_1v_face(p)
}

pub fn is_0_face(p: &Point2d) -> bool {
    p.x() % 2 == 1 && p.y() % 2 == 1
}
