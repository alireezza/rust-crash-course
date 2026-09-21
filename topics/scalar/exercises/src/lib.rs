pub fn eq(a: &str, b: &str) -> bool {
    a == b
}
fn main() {
    let result = eq("hello", "hello");
    println!("Are equal: {}", result);
}

pub fn add() {
    todo!();
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x + y + z
}
