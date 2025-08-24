pub fn calc_n_fibonacci_number(n: i32) -> i32 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return calc_n_fibonacci_number(n-1) + calc_n_fibonacci_number(n-2)
    }
}