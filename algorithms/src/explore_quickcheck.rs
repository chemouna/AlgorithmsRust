extern crate quickcheck;

use quickcheck::{TestResult, quickcheck};

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

fn main() {
    fn prop(xs: Vec<isize>) -> TestResult {
        if xs.len() != 1 {
            return TestResult::discard()
        }
        TestResult::from_bool(xs == reverse(&*xs))
    }
    quickcheck(prop as fn(Vec<isize>) -> TestResult);
}
