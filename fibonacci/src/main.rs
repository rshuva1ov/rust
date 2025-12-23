use std::io;

fn main() {
    println!("=== Генератор чисел Фибоначчи ===");
    println!("Введите номер числа Фибоначчи (n):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let n: u64 = input.trim().parse().expect("Пожалуйста, введите положительное число");

    let result = fibonacci(n);
    println!("{}-е число Фибоначчи: {}", n, result);

    let sequence = fibonacci_sequence(n);
    let formatted = sequence
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    println!("Все значения до n: {}", formatted);
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn fibonacci_sequence(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut sequence = Vec::with_capacity((n + 1) as usize);
    sequence.push(0);
    sequence.push(1);

    for _ in 2..=n {
        let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
        sequence.push(next);
    }

    sequence
}
