use stopwatch::Stopwatch;

pub fn is_prime(number: i32) -> bool {
    if number % 2 == 0 {
        return false;
    }

    let mut index = 1;
    let mut count = 1;
    let sqrt_number = (number as f32).sqrt().floor() as i32;

    while index <= sqrt_number {
        if number % index == 0 {
            count += 1;
        }
        if count > 2 {
            return false;
        }
        index += 2;
    }
    true
}

pub fn list_primes(number: i32) -> Vec<i32> {
    let mut primes: Vec<i32> = Vec::new();
    let mut index = 1;

    while index < number {
        if is_prime(index) {
            primes.push(index)
        }
        index += 2;
    }

    primes
}

pub fn count_primes(number: i32) -> i32 {
    let mut index = 1;
    let mut count = 0;

    while index < number {
        if is_prime(index) {
            count += 1;
        }
        index += 2;
    }
    count
}

fn main() {
    let sw = Stopwatch::start_new();
    let test_number = 1_000_000;
    println!("{:?}", list_primes(test_number));
    println!(
        "There are {:?} primes up to {test_number}.",
        count_primes(test_number)
    );
    println!("This took {}ms to run.", sw.elapsed_ms());
}
