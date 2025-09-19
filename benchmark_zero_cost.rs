use std::time::Instant;

fn iterator_approach(data: &[i32]) -> i32 {
    data.iter()
        .map(|x| x.wrapping_mul(x.wrapping_add(1)))  // Some computation
        .filter(|&x| x % 3 == 0)
        .sum()
}

fn manual_approach(data: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &x in data {
        let result = x.wrapping_mul(x.wrapping_add(1));
        if result % 3 == 0 {
            sum = sum.wrapping_add(result);
        }
    }
    sum
}

fn main() {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // Warm up
    iterator_approach(&data);
    manual_approach(&data);
    
    // Benchmark iterator approach
    let start = Instant::now();
    let result1 = iterator_approach(&data);
    let iter_time = start.elapsed();
    
    // Benchmark manual approach  
    let start = Instant::now();
    let result2 = manual_approach(&data);
    let manual_time = start.elapsed();
    
    println!("Iterator result: {}", result1);
    println!("Manual result: {}", result2);
    println!("Results match: {}", result1 == result2);
    println!("Iterator time: {:?}", iter_time);
    println!("Manual time: {:?}", manual_time);
    println!("Performance ratio: {:.2}", 
             iter_time.as_nanos() as f64 / manual_time.as_nanos() as f64);
}