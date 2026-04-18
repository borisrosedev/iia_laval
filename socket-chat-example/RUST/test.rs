
// Test pour voir comment fonctionne Rust

fn main() {
    println!("Hello, Rust!");
    
    let x = 42;
    let y = "world";
    
    println!("Number: {}", x);
    println!("String: {}", y);
    
    let result = add(10, 20);
    println!("Sum: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}