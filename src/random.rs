use rand::{thread_rng, Rng};

pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

    // Exclusive range
    rng.gen_range(min..max)
}

#[cfg(test)]
mod tests {
    use super::random_range;
    #[test]
    fn it_works() {
        println!("{}", random_range(0, 10));
    }
}