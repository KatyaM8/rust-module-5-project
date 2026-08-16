pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|value| value % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов без промежуточного буфера.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|byte| **byte != 0).count()
}

/// Удаляет все пробельные символы и приводит строку к нижнему регистру.
pub fn normalize(input: &str) -> String {
    if input.is_ascii() {
        let bytes = input
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .map(|byte| byte.to_ascii_lowercase())
            .collect();
        return String::from_utf8(bytes).expect("ASCII input always produces valid UTF-8");
    }

    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Среднее арифметическое положительных значений.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .copied()
        .filter(|value| *value > 0)
        .fold((0_i64, 0_usize), |(sum, count), value| {
            (sum + value, count + 1)
        });
    if count == 0 {
        return 0.0;
    }
    sum as f64 / count as f64
}

/// Складывает значение с самим собой, пока владелец памяти ещё жив.
pub fn use_after_free() -> i32 {
    let value = Box::new(42_i32);
    *value + *value
}
