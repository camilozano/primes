
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


fn main() {
    let val = sieve(101,500);
    println!("{:?}",val.len());

}