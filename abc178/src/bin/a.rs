use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x : i64,
    }
    if x == 0 {
        println!{"{}",1};
    } else {
        println!{"{}",0};
    }
}
