use broken_app::{algo, leak_buffer, normalize, sum_even, use_after_free};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn sums_even_for_empty_and_single_value_slices() {
    assert_eq!(sum_even(&[]), 0);
    assert_eq!(sum_even(&[2]), 2);
    assert_eq!(sum_even(&[3]), 0);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn counts_non_zero_bytes_without_losing_the_buffer() {
    let data = vec![1_u8; 4_096];
    assert_eq!(leak_buffer(&data), data.len());
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_removes_all_whitespace() {
    assert_eq!(normalize(" Hello\tRust\nWorld "), "hellorustworld");
    assert_eq!(normalize(" Ä\u{2003}Ö "), "äö");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    // Ожидается (5 + 15) / 2 = 10, но текущая реализация делит на все элементы.
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn average_positive_handles_no_positive_values() {
    assert_eq!(broken_app::average_positive(&[]), 0.0);
    assert_eq!(broken_app::average_positive(&[-5, 0, -1]), 0.0);
}

#[test]
fn freed_value_is_not_read_again() {
    assert_eq!(use_after_free(), 84);
}

#[test]
fn concurrent_increment_does_not_lose_updates() {
    let iterations = if cfg!(miri) { 100 } else { 25_000 };
    let total = broken_app::concurrency::race_increment(iterations, 8);
    assert_eq!(total, (iterations * 8) as u64);
}
