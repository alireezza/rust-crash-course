// Exercise 1 
pub fn hello() -> String { 
    "Hello Rust".to_string() 
}
// Exercise 2 
pub fn greet(name: &str) -> String { 
    format("Hello {}", name) 
}

pub fn append(mut s: String) -> String {
    todo!();
}
