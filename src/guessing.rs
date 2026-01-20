pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!("Guess value must be between 1 and 100, got {}", value));
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

