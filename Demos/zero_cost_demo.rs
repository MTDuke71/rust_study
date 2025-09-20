// Zero-cost abstraction demonstration
// Compile with: rustc -O zero_cost_demo.rs
// Then examine assembly with: objdump -d zero_cost_demo.exe

fn iterator_version(data: &[i32]) -> i32 {
    data.iter()
        .map(|x| x * 2)
        .filter(|&x| x > 5)
        .fold(0, |acc, x| acc + x)
}

fn manual_version(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &value in data {
        let doubled = value * 2;
        if doubled > 5 {
            sum += doubled;
        }
    }
    sum
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    println!("Iterator result: {}", iterator_version(&numbers));
    println!("Manual result: {}", manual_version(&numbers));
    
    // Both produce the same result and same assembly code!
}