mod test_affix_update;

#[macro_export]
macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr) => {
        match (&$lhs, &$rhs) {
            (left_val, right_val) => {
                assert!(
                    (left_val - right_val).abs() < left_val / 1_000_000.0,
                    "left: {}, right: {}",
                    left_val,
                    right_val
                );
            }
        }
    };
}
