use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


fn sieve(from:usize, to:usize) -> Vec<usize>{

    let mut is_prime = vec![true; to+1];
    let sqrtlmt = (to as f64).sqrt() as usize +1;

    let mut min = from;
    if to > 2 {
        is_prime[0] = false;
        is_prime[1] = false;
        min = 2;
    }

    for idx in min..sqrtlmt {
        if is_prime[idx]{
            let mut multiple = idx*idx;
            while multiple <= to {
                is_prime[multiple] = false;
                multiple += idx;
            }
        }
    }

    let mut prime_list = Vec::new();
    for i in from..is_prime.len() {
        if is_prime[i] {
            prime_list.push(i);
        }
    }
    prime_list
}

static NTHREADS: i32 = 3;
fn main() {
    
    let upper = 100000000;
    let div = upper/(NTHREADS as usize);
    let mut count = 0;


    for i in (0..upper).step_by(div){
        let from = i;
        let to = i+div-1;
        count += sieve(from, to).len();
    }

    println!("{}",count);



}