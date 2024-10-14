// quiz3.rs
// This is a quiz for the following sections:
// - Tests

// This quiz isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests that we get the result
// we expect to get when we call `times_two` with a negative number.
// No hints, you can do this :)

// I AM DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4)); // Перевіряємо, що 4 є парним числом
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5)); // Перевіряємо, що 5 є непарним числом
    }
}
