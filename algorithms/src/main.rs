#[cfg(test)]
#[macro_use]

extern crate quickcheck;

/*fn main() {
    let arr1: [f64; 9] = [-2.0, 1.0, -3.0, 4.0, -1.0, 2.0, 1.0, -5.0, 4.0];
    let (max, start, end) = max_subarray_kadane(&arr1);
    println!("{}, {}, {}", max, start, end);

    let arr2: [f64; 1] = [3.0];
    let (max2, start2, end2) = max_subarray_kadane(&arr2);
    println!("{}, {}, {}", max2, start2, end2);

    let arr3: [f64; 2] = [-2.0, 1.0];
    let (max3, start3, end3) = max_subarray_kadane(&arr3);
    println!("{}, {}, {}", max3, start3, end3);

    let arr4: [f64; 1] = [-2.0];
    let (max4, start4, end4) = max_subarray_kadane(&arr4);
    println!("{}, {}, {}", max4, start4, end4);
}*/

#[cfg(test)]
mod test {
    use quickcheck::TestResult;

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


    quickcheck! {
        fn prop_empty_vector(xs: &[f64]) -> TestResult {
            if xs.len() != 1 {
                return TestResult::discard()
            }
            TestResult::from_bool((0, 0, 0) == max_subarray_kadane(&[]))
        }
    }
}

