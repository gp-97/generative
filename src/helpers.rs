use crate::Point;

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

pub fn euclid_dist(p1: &Point, p2: &Point) -> f32 {
    ((p1.get_x() - p2.get_x()).powf(2.0) + (p1.get_y() - p2.get_y()).powf(2.0)).powf(0.5)
}

fn merge_sort(arr: &mut Vec<Point>, start: usize, end: usize, temp_arr: &mut Vec<Point>) {
    if start < end {
        let mid = start + (end - start) / 2;
        merge_sort(arr, start, mid, temp_arr);
        merge_sort(arr, mid + 1, end, temp_arr);

        let mut i = start;
        let mut j = end;
        let mut k = start;

        while i <= mid {
            temp_arr[i] = arr[i];
            i += 1;
        }
        while j > mid {
            temp_arr[i] = arr[j];
            i += 1;
            j -= 1;
        }
        i = start;
        j = end;
        while k <= end {
            if temp_arr[i] < temp_arr[j] {
                arr[k] = temp_arr[i];
                i += 1;
            } else {
                arr[k] = temp_arr[j];
                j -= 1;
            }
            k += 1;
        }
    }
}

pub fn sort(arr: &mut Vec<Point>) {
    let mut temp_arr = arr.clone();
    let start = 0;
    let end = arr.len() - 1;
    merge_sort(arr, start, end, &mut temp_arr);
}
