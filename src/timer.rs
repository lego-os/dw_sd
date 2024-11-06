pub struct CountDown {
    deadline: usize,
    get_milli: fn() -> usize,
}

impl CountDown {
    pub fn new(millis: usize, get_milli: fn() -> usize) -> Self {
        let now = get_milli();
        Self {
            deadline: millis + now,
            get_milli,
        }
    }

    pub fn timeout(&self) -> bool {
        (self.get_milli)() > self.deadline
    }
}