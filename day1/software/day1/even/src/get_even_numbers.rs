pub fn get_even_numbers(numbers: &[i32]) -> String {
    let mut filtered: Vec<i32> = numbers.iter().filter(|num| *num % 2 == 0).cloned().collect();
    filtered.sort();
    let strings: Vec<String> = filtered.iter().map(|i| i.to_string()).collect();
    strings.join(" - ")
}