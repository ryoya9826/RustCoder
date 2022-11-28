use proconio::{fastout, input};

fn main() {
    input! {
        s: String,
    }
    let mut cnt = 0;
    for x in s.chars()
    { if x == 'v' { cnt += 1} else { cnt += 2}}
    println!("{}", cnt);
}