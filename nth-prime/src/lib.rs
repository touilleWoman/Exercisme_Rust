fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => (factorial(num - 1)) * num,
    }
}

fn is_prime(p: u128) -> bool {
    //Wilson's theorem
    if (factorial(p - 1) % p) == (p - 1) % p {
        true
    } else {
        false
    }
}

pub fn nth(n: u32) -> u32 {
    let mut p = 2;
    let mut count = 0;
    while count != n {
        p += 1;
        if is_prime(p as u128) {
            count += 1;
        }
    }
    p
}
