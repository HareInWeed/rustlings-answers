// iterators4.rs


// Complete this function to return the factorial of num
// Do not use:
// - return
// Try not to use:
// - imperative style loops (for, while)
// - additional variables
// For an extra challenge, don't use:
// - recursion
// Execute `rustlings hint iterators4` for hints.

// loop
pub fn factorial_1(num: u64) -> u64 {
    let mut f = 1;
    for n in 1..(num + 1) {
        f *= n;
    }
    f
}

// recursion
pub fn factorial_2(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        num * factorial_2(num - 1)
    }
}

// iter + fold
pub fn factorial_3(num: u64) -> u64 {
    (1..(num + 1)).fold(1, |f, n| f * n)
}

static factorial: fn(u64) -> u64 = factorial_3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
