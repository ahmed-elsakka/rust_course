pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Counter {
    value: u32
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn enter_value_more_than_0(n: u64) {
    if n <= 0 {
        panic!("You entered a value less than or equal zero");
    }
    println!("{}", n);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn counter_works() {
        let mut counter = Counter {value: 0};
        counter.increment();
        assert_ne!(counter.value, 0);
    }

    #[test]
    fn is_even_works() {
        let input = 3;
        assert!(!is_even(input));
    }

    #[test]
    #[should_panic]
    fn function_panics() {
        enter_value_more_than_0(0);
    }
}
