pub struct Controller {
    state: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self { state: false }
    }

    pub fn toggle(&mut self) -> bool {
        self.state = !self.state;
        self.state
    }

    pub fn state(&self) -> bool {
        self.state
    }
}