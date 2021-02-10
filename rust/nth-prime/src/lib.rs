pub fn nth(n: u32) -> u32 {
    let mut current_candidate: u32 = 2;
    let mut primes: Vec<u32> = Vec::new();
    let mut len: u32 = 0;
    
    // How to access primes.len() in the while loop?
    // I got stuck because of getting "borrow of moved value" error
    while len <= n {
        let mut divisible: bool = false;
        for i in primes.iter() {
            if current_candidate % i == 0 {
                divisible = true;
                break;
            }
        }

        if !divisible {
            primes.push(current_candidate);
            len += 1;
        }

        current_candidate += 1;
    }

    return primes.last().copied().unwrap();
}
