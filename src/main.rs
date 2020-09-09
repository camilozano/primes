

fn block_sieve(from: i32, to: i32)-> (i32,Vec<bool>) {

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
    for i in 0..memory_size {
        found += is_prime[i] as i32;
    }
    // if from <= 2 {found += 1}

    (found,is_prime)
}

fn main(){
    let from = 0;
    let to = 500;
    let res = block_sieve(from, to);

    let from1 = 501;
    let to1 = 1000;
    let res1 = block_sieve(from1, to1);
    println!("{} {} {}",res.0, res1.0, res.0+res1.0);

    let printer = |val: Vec<bool> , f: i32| {
        for i in 1..val.len() {
            if val[i] {print!("{} ", i+(f as usize))}
        }
        println!();
        println!();
    };

    printer(res.1, from);
    printer(res1.1, from1);


}