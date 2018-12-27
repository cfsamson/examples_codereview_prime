use codereview_prime as prime;
#[test]
fn small_number_checks() {
    assert_eq!(prime::is_prime(11), true);
    assert_eq!(prime::is_prime(12), false);
    assert_eq!(prime::is_prime(13), true);
}

#[test]
fn large_number_checks() {
    assert_eq!(prime::is_prime(179434027), true);
    assert_eq!(prime::is_prime(179434029), false);
    assert_eq!(prime::is_prime(179434031), false);
    assert_eq!(prime::is_prime(179434033), true);
}

#[test]
fn first_10_numbers_tested() {
    assert_eq!(prime::is_prime(0), false);
    assert_eq!(prime::is_prime(1), false);
    assert_eq!(prime::is_prime(2), true);
    assert_eq!(prime::is_prime(3), true);
    assert_eq!(prime::is_prime(4), false);
    assert_eq!(prime::is_prime(5), true);
    assert_eq!(prime::is_prime(6), false);
    assert_eq!(prime::is_prime(7), true);
    assert_eq!(prime::is_prime(8), false);
    assert_eq!(prime::is_prime(9), false);
    assert_eq!(prime::is_prime(10), false);
}
