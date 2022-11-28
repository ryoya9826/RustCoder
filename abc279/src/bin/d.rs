use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let cnt:f64 = (a/(2.*b)).powf(2./3.) -1.;
    let n = cnt.round();
    let ans: f64;
    if n <= 0. {
        let ans1 = a;
        let ans2 = a/((2_f64).powf(0.5)) + b;
        if ans1 <= ans2 {ans = ans1;} else {ans = ans2;}
    } else {
        ans = (a/(n+1.))*((n+1.).powf(0.5)) + n*b;
    }
    println!("{}",ans);
}
