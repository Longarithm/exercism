pub fn nth(n: u32) -> u32 {
    let mut current_candidate: u32 = 2;
    let mut primes: Vec<u32> = Vec::new();
    
    while primes.len() <= n as usize {
        let mut divisible: bool = false;
        for i in primes.iter() {
            if current_candidate % i == 0 {
                divisible = true;
                break;
            }
        }

        if !divisible {
            primes.push(current_candidate);
        }

        current_candidate += 1;
    }

    return primes.last().copied().unwrap();
}
