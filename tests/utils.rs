#[macro_use]
extern crate more_asserts;

extern crate stars_rs;

#[cfg(test)]
mod utils {
    use super::stars_rs::utils::random_in_range;

    #[test]
    fn test_random_in_range() {
        let subject = random_in_range(0.0, 1.0);
        assert_ge!(subject, 0.0);
        assert_le!(subject, 1.0);
    }
}
