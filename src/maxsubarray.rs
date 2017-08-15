fn max_subarray_kadane(arr: &[f64]) -> (f64, usize, usize) {
    if arr.len() == 0 {
        panic!("Error: empty array")
    }

    let mut max_ending_here: f64 = arr[0];
    let mut max_so_far: f64 = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut potential_start = 0;

    for i in 0..arr.len() {
        if arr[i] > max_ending_here + arr[i] {
            potential_start = i;
            max_ending_here = arr[i];
        }
        else {
            max_ending_here += arr[i];
        }

        if max_ending_here > max_so_far {
            max_so_far = max_ending_here;
            end = i;
            start = potential_start;
        }
    }
    (max_so_far, start, end)
}

fn main() {
    let arr1: [f64; 9] = [-2.0, 1.0, -3.0, 4.0, -1.0, 2.0, 1.0, -5.0, 4.0];
    let (max, start, end) = max_subarray_kadane(&arr1);
    println!("{}, {}, {}", max, start, end);
}
