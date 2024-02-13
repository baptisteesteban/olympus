use crate::RGB;

#[test]
fn test_creation() {
    let c = RGB::new(7, 25, 11);
    assert_eq!(c.r(), 7);
    assert_eq!(c.g(), 25);
    assert_eq!(c.b(), 11);
}

#[test]
fn test_display() {
    let c = RGB::new(7, 25, 11);
    let formatted = String::from(format!("{}", c).as_str());
    assert_eq!(formatted, "(7, 25, 11)");
}
