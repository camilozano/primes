use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

static NTHREADS: i32 = 8;
static UPPER_LIMIT: usize = 100000000;

fn sieve(from:usize, to:usize, tx: Sender<usize>) -> i32 {

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

    let mut count = 0;
    for i in from..is_prime.len() {
        if is_prime[i] {
            count += 1;
            tx.send(i).unwrap();
        }
    }
    count
}

fn prime_runner(upper: usize) -> u32 {

    let div = upper/(NTHREADS as usize);
    let mut count: u32 = 0;
    let mut sum: u64 = 0;

    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    let mut children = Vec::new();

    let start = Instant::now();

    for i in (0..upper).step_by(div){
        let thread_tx = tx.clone();
        let from = i;
        let to = i+div-1;

        let child = thread::spawn(move || {
            sieve(from,to,thread_tx);
            // println!("{}\tThread {} finished", count,i);
        });
        children.push(child);
    }


    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    let elapsed_time = start.elapsed().as_secs_f32();

    

    let mut list = Vec::new();
    drop(tx);
    for val in rx{
        count += 1;
        sum += val as u64;
        list.push(val)
    }

    list.sort();

    println!("{} {} {}\n{:?}",elapsed_time,count,sum,&list[list.len()-10..]);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime_check_10_to_8th(){
        assert_eq!(prime_runner(100000000),5761455);
    }
}


fn main() {
    
    prime_runner(UPPER_LIMIT); 

 }




