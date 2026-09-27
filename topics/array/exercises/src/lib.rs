// Exercise 1 
pub fn zeros() -> [u32; 100] {
    [0; 100]
}

fn main() {
    let arr = zeros();
    println!("arr[0]: {}", arr[0]);
}

// Exercise 2 
pub fn first_3(s: &[u32]) -> &[u32] { 
    &s[..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    todo!();
}
