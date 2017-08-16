#[cfg(test)]
#[macro_use]

extern crate quickcheck;

use quickcheck::quickcheck;

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}

/*fn prop_reverse_idempotent(xs: Vec<u32>) -> bool {
    xs == reverse(&reverse(&xs))
}*/

#[cfg(test)]
mod tests {
    quickcheck! {
        fn prop(xs: Vec<isize>) -> TestResult {
            if xs.len() != 1 {
                return TestResult::discard()
            }
            TestResult::from_bool(xs == reverse(&*xs))
        }
    }

    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        (
            (b.0 - a.0).powi(2) +
                (b.1 - a.1).powi(2)
        ).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
    }


}
