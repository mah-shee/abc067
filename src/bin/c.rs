#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let mut ans = std::isize::MAX;
    let mut an = vec![0; n];
    let mut sum = 0;
    an[0] = a[0];
    sum += a[n - 1];
    for i in 0..n - 1 {
        an[i + 1] = an[i] + a[i + 1];
        sum += a[i];
    }
    for i in 0..n - 1 {
        let diff = (sum - 2 * an[i]).abs();
        ans = min(ans, diff);
    }
    println!("{}", ans);
}
