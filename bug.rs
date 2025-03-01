fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let index = 5;

    // This will panic at runtime if index is out of bounds
    let value = vec[index];
    println!("Value at index {}: {}", index, value);
}