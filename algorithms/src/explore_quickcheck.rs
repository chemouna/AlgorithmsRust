#[cfg(test)]
#[macro_use]

extern crate quickcheck;

#[cfg(test)]
mod tests {
    use quickcheck::TestResult;

    fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
        let mut rev = vec!();
        for x in xs.iter() {
            rev.insert(0, x.clone())
        }
        rev
    }

    quickcheck! {
        fn prop_one_element_vector_same(xs: Vec<isize>) -> TestResult {
            if xs.len() != 1 {
                return TestResult::discard()
            }
            TestResult::from_bool(xs == reverse(&*xs))
        }
    }

}
