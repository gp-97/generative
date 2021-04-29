pub fn linspace(start: f32, end: f32, npoints: u32) -> Vec<f32> {
    let diff = (end - start) / (npoints as f32 - 1.0);
    let mut points = vec![start];
    for i in 1..(npoints - 1) {
        let next = points[i as usize - 1] + diff;
        points.push(next);
    }
    points.push(end);
    points
}

fn factorial(n: usize) -> usize {
    let mut fact = 1_usize;
    for i in 1..=n {
        fact *= i as usize;
    }
    fact
}

pub fn comb(n: usize, r: usize) -> usize {
    factorial(n) / (factorial(n - r) * factorial(r))
}
