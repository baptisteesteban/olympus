use crate::Box2d;

#[test]
fn test_creation_and_attributes() {
    let domain = Box2d::new_from_dimension(7, 3);
    assert_eq!(domain.width(), 7);
    assert_eq!(domain.height(), 3);
}

#[test]
fn test_iter() {
    let ref_p = [
        "(0, 0)", "(1, 0)", "(2, 0)", "(3, 0)", "(0, 1)", "(1, 1)", "(2, 1)", "(3, 1)",
    ];

    let domain = Box2d::new_from_dimension(4, 2);
    let mut i = 0;
    for p in domain {
        let formatted = String::from(format!("{}", p).as_str());
        assert_eq!(formatted, ref_p[i]);
        i += 1;
    }
}
