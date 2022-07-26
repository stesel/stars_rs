#[macro_use]
extern crate more_asserts;

extern crate stars_rs;

#[cfg(test)]
mod utils {
    use stars_rs::utils::{random_in_range, random_in_rect_edge};

    #[test]
    fn test_random_in_range() {
        for _ in 0..10 {
            let subject = random_in_range(0.0, 1.0);
            assert_ge!(subject, 0.0);
            assert_le!(subject, 1.0);
        }
    }

    #[test]
    fn test_random_in_rect_edge() {
        for _ in 0..10 {
            let subject = random_in_rect_edge(-1.0, 1.0, 1.0, -1.0);
            assert_ge!(subject.x, -1.0);
            assert_le!(subject.x, 1.0);
            assert_ge!(subject.y, -1.0);
            assert_le!(subject.y, 1.0);
        }
    }
}
