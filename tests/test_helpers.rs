#[cfg(test)]
use generative::helpers::*;
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
