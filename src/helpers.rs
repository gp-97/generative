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

pub fn map(val: f32, start_1: f32, end_1: f32, start_2: f32, end_2: f32, clamp: bool) -> f32 {
    if start_1 == end_1 {
        val
    } else {
        let new_val = ((val - start_1) / (end_1 - start_1)) * (end_2 - start_2) + start_2;
        if !clamp {
            return new_val;
        }
        if start_2 < end_2 {
            f32::clamp(new_val, start_2, end_2)
        } else {
            f32::clamp(new_val, end_2, start_2)
        }
    }
}

pub fn lerp(start: f32, end: f32, amt: f32) -> f32 {
    start * (1.0 - amt) + end * amt
}

pub fn norm(val: f32, start: f32, end: f32) -> f32 {
    map(val, start, end, 0.0, 1.0, false)
}
