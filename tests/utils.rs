#[macro_use]
extern crate more_asserts;

extern crate stars_rs;

#[cfg(test)]
mod utils {
    use stars_rs::utils::{hit_test, random_in_range, random_in_rect_edge, BoundingRect};

    #[test]
    fn test_hit_test() {
        let lhr: BoundingRect = BoundingRect {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        };
        let rhr: BoundingRect = BoundingRect {
            x: 0.49,
            y: 0.49,
            width: 1.0,
            height: 1.0,
        };
        assert_eq!(hit_test(lhr, rhr), true);

        let lhr: BoundingRect = BoundingRect {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        };
        let rhr: BoundingRect = BoundingRect {
            x: 0.5,
            y: 0.5,
            width: 1.0,
            height: 1.0,
        };
        assert_eq!(hit_test(lhr, rhr), false);
    }

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
