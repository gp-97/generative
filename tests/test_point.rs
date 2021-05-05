#[cfg(test)]
use generative::Point;
#[test]
fn test_point_init() {
    let p1 = Point::new(5.0, 6.0);
    assert_eq!(p1.get_x(), 5.0);
    assert_eq!(p1.get_y(), 6.0);
}
#[test]
fn test_from() {
    let p1 = Point::from((5.0, 6.0));
    assert_eq!(p1.get_x(), 5.0);
    assert_eq!(p1.get_y(), 6.0);
}
#[test]
fn test_eq() {
    let p1 = Point::new(5.0, 6.0);
    let p2 = Point::from((5.0, 6.0));
    assert_eq!(p1, p2);
}
#[test]
fn test_ord() {
    let p1 = Point::new(5.0, 6.0);
    let p2 = Point::new(2.0, 3.0);
    assert_eq!(p1 < p2, false);
}
#[test]
fn test_unit_vec() {
    let p1 = Point::new(3.0, 4.0);
    assert_eq!(p1.get_unit_vec(), Point::from((0.6, 0.8)));
}
