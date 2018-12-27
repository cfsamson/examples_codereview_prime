/// This function takes a 64-bit unsigned integer and checks if it is a prime.
///
/// If the number is prime, `true` is returned, and vice-versa.
///
/// #Example
///
/// ```rust
/// use codereview_prime as prime;
///
/// let result = prime::is_prime(31);
/// assert_eq!(result, true);
/// ```
pub fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 || num == 3 {
        return true;
    }
    // Even numbers and multiples of 3 are eliminated
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    // Optimized divisor approach
    // First we calculate the maximum limit of iteration
    let limit = (num as f64).sqrt() as u64;
    // We start the iteration from 5 (2 and 3 have been already tested)
    let mut divisor = 5;
    // The step alternates between 2 and 4 to keep the divisor of the form
    // 6k +/- 1, where k is an integer
    let mut step = 2;
    while divisor <= limit {
        if num % divisor == 0 {
            return false;
        }
        divisor += step;
        step = if step == 2 { 4 } else { 2 }
    }
    true
}
