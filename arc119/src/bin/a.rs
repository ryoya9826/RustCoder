use proconio::{fastout, input};
use libm::log2;

#[fastout]
fn main() {
    input! {n:i64,}
    let B:i64 = log2(n as f64).round() as i64;
    let mut ans =Vec::new();
    for b in 0..B {
        if b == 0 { ans.push(n) } 
        else { 
            let a:i64 =  ((n as f64)/(2_f64).powf(b as f64)).floor() as i64;
            ans.push((1-2_i64.pow(b as u32))*a+b+n)
        }
    }
    let min;
    match ans.iter().min() {
        Some(n) => min = *n,
        None => min = 1,
    }
    println!("{}",min)
}
