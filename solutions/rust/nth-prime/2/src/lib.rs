pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut number = 2;

    loop {
        if is_prime(number) {
            if count == n {
                return number;
            }

            count += 1;
        }

        number += 1;
    }
}

fn is_prime(number: u32) -> bool {
    if number < 2 {
        return false;
    }

    let mut divisor = 2;

    while divisor * divisor <= number {
        if number.is_multiple_of(divisor) {
            return false;
        }

        divisor += 1;
    }

    true
}