// Простые примеры для иллюстрации концепции владения в Rust.

fn main() {
    move_semantics();
    copy_types();
    borrowing();
    mutable_borrow();
    ownership_transfer_back();
}

fn move_semantics() {
    // String хранится в куче, поэтому при присваивании право владения перемещается (move)
    let greeting = String::from("hello");
    let moved = greeting;

    // println!("{greeting}") // <- не скомпилируется: greeting больше не владелец
    println!("Moved value: {moved}");
}

fn copy_types() {
    // i32 реализует Copy, поэтому при присваивании значение копируется, а не перемещается
    let x: i32 = 42;
    let y = x;

    println!("x is still usable: {x}, y is a copy: {y}");
}

fn borrowing() {
    let phrase = String::from("borrowed phrase");
    // Передаем неизменяемую ссылку, владение остается у caller
    print_length(&phrase);

    // phrase все еще доступна
    println!("Still own it: {phrase}");
}

fn mutable_borrow() {
    let mut label = String::from("prefix");
    // Одна изменяемая ссылка одновременно — гарантирует отсутствие data races
    add_suffix(&mut label);

    println!("After mutation: {label}");
}

fn ownership_transfer_back() {
    let original = String::from("original");
    // Функция принимает владение и возвращает его обратно
    let returned = take_and_return(original);

    println!("Got it back: {returned}");
}

fn print_length(value: &String) {
    println!("Len = {}", value.len());
}

fn add_suffix(value: &mut String) {
    value.push_str(" + suffix");
}

fn take_and_return(value: String) -> String {
    // Здесь value владеет данными; возвращаем для передачи владения caller
    value
}
