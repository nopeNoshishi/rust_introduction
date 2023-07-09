use toy_vec::ToyVec;

fn main() -> std::io::Result<()> {
    let mut numbers = ToyVec::new();

    for i in 2..10 {
        numbers.push(i);

        if !is_prime(i) {
            numbers.pop();
        }
    }

    for n in &numbers {
        println!("Prime: {}", n)
    }

    Ok(())
}

fn is_prime(n: u64) -> bool {
    let lim = (n as f64).sqrt().floor() as u64 + 1;

    for i in 2..lim {
        if n % i == 0 {
            return false;
        }
    }

    true
}
