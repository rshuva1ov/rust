use std::io;

fn main() {
    println!("=== Конвертер температуры ===");
    println!("Выберите тип конвертации:");
    println!("1. Фаренгейт → Цельсий");
    println!("2. Цельсий → Фаренгейт");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Не удалось прочитать строку");

    let choice: u32 = choice.trim().parse().expect("Пожалуйста, введите число 1 или 2");

    println!("Введите температуру:");

    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Не удалось прочитать строку");

    let temperature: f64 = temperature.trim().parse().expect("Пожалуйста, введите число");

    match choice {
        1 => {
            let celsius = fahrenheit_to_celsius(temperature);
            println!("{:.2}°F = {:.2}°C", temperature, celsius);
        }
        2 => {
            let fahrenheit = celsius_to_fahrenheit(temperature);
            println!("{:.2}°C = {:.2}°F", temperature, fahrenheit);
        }
        _ => {
            println!("Неверный выбор. Пожалуйста, выберите 1 или 2.");
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
