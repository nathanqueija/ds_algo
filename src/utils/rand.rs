use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn generate_random_number(max: usize) -> usize {
    RG.lock().unwrap().next_value(max)
}

pub struct RandGen {
    current: usize,
    multiply: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(current: usize) -> Self {
        RandGen {
            current,
            multiply: 56394237,
            inc: 34642349,
            modulo: 23254544563,
        }
    }

    pub fn next_value(&mut self, max_value: usize) -> usize {
        self.current = (self.current * self.multiply + self.inc) % self.modulo;
        self.current % max_value
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::rand::RandGen;

    #[test]
    fn test_rands_printout() {
        let mut r = RandGen::new(12);
        for _ in 1..100 {
            println!("----{}", r.next_value(100));
        }
    }
}
