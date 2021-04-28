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
