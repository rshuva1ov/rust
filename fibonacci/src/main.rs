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
