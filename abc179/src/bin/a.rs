use proconio::{fastout, input};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        S:String,
    }
    println!("{}",if S.chars().nth(S.len()-1) == Some('s') {S+"es"} else {S+"s" })
}
