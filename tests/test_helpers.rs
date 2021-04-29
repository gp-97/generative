#[cfg(test)]
use generative::helpers::linspace;
#[test]
fn test_linspace() {
    assert_eq!(linspace(2.0, 3.0, 5), vec![2.0, 2.25, 2.5, 2.75, 3.0]);
}
