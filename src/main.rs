use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

static NTHREADS: i32 = 64;
static UPPER_LIMIT: i32 = 100000000;

fn block_sieve(from: i32, to: i32, tx: Sender<usize>)-> i32 {

    let memory_size = ((to-from+1)/2) as usize;
    let mut is_prime: Vec<bool> = vec![true;memory_size];

    // Maybe +1 check line 11
    let range = (to as f64).sqrt() as i32 + 1;
    
    for i in (3..range).step_by(2){
        if i >= 3*3 && i%3 == 0 {continue}
        if i >= 5*5 && i%5 == 0 {continue}
        if i >= 7*7 && i%7 == 0 {continue}
        if i >= 11*11 && i%11 == 0 {continue}
        if i >= 13*13 && i%13 == 0 {continue}
        
        let mut min = ((from+i-1)/i)*i;
        if min < i*i {min = i*i}
        if (min & 1) == 0 {min += i}

        let mut j = min;
        while j <= to{
            let index = ((j-from)/2) as usize;
            is_prime[index] = false;
            j+= 2*i;
        }


    }

    let mut found = 0;
    let idx_fix = ((from&1)+1)%2;

    // let mut prime_list: Vec<i32> = Vec::new();
    for i in 0..memory_size {
        if is_prime[i] {
            found += is_prime[i] as i32;
            let actual_number = from + 2*(i as i32) + idx_fix;
            tx.send(actual_number as usize).unwrap();
            // prime_list.push(actual_number);
        }
    }

    found
}

fn prime_runner(upper: i32) -> u32 {

    let div: i32 = upper/NTHREADS;
    let mut count: u32 = 0;
    let mut sum: u64 = 0;

    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    let mut children = Vec::new();

    let start = Instant::now();

    for i in (0..upper).step_by(div as usize){
        let thread_tx = tx.clone();
        let from: i32 = i as i32;
        let to = i+div-1;

        let child = thread::spawn(move || {
            block_sieve(from,to,thread_tx);
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




