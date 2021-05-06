#[cfg(test)]
use generative::helpers::*;
use generative::Point;
#[test]
fn test_linspace() {
    assert_eq!(linspace(2.0, 3.0, 5), vec![2.0, 2.25, 2.5, 2.75, 3.0]);
}
#[test]
fn test_factorial() {
    assert_eq!(comb(5, 2), 10);
}
#[test]
fn test_map() {
    assert_eq!(map(20.0, 0.0, 100.0, 40.0, 50.0, false), 42.0);
}
#[test]
fn test_lerp() {
    assert_eq!(lerp(20.0, 100.0, 0.2), 36.0);
}
#[test]
fn test_norm() {
    assert_eq!(norm(50.0, 0.0, 100.0), 0.5);
}
#[test]
fn test_euclid() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    assert_eq!(euclid_dist(&p1, &p2), 5.0);
}
#[test]
fn test_sort() {
    let mut vec = vec![
        Point::new(10.0, 10.0),
        Point::new(9.0, 9.0),
        Point::new(8.0, 8.0),
        Point::new(7.0, 7.0),
        Point::new(6.0, 6.0),
        Point::new(5.0, 5.0),
        Point::new(4.0, 4.0),
        Point::new(3.0, 3.0),
        Point::new(2.0, 2.0),
        Point::new(1.0, 1.0),
        Point::new(0.0, 0.0),
    ];
    let test_vec = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.0),
        Point::new(3.0, 3.0),
        Point::new(4.0, 4.0),
        Point::new(5.0, 5.0),
        Point::new(6.0, 6.0),
        Point::new(7.0, 7.0),
        Point::new(8.0, 8.0),
        Point::new(9.0, 9.0),
        Point::new(10.0, 10.0),
    ];
    sort(&mut vec);
    for i in 0..vec.len() {
        assert_eq!(vec[i], test_vec[i]);
    }
}
