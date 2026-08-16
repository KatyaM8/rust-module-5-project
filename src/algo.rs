/// Возвращает отсортированные уникальные значения.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

/// Вычисляет число Фибоначчи за линейное время и с постоянной памятью.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut previous = 0;
            let mut current = 1;
            for _ in 2..=n {
                (previous, current) = (current, previous + current);
            }
            current
        }
    }
}
