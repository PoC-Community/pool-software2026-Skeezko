mod get_even_numbers;

fn main() {
    let array = vec![1, 0, 19, 17, 16, 8, 13, 24];
    println!("{}", get_even_numbers::get_even_numbers(&array));
}
