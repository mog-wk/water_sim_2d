pub struct Controller {
    pub paused: bool,
    pub shiftmod: bool,
    pub ctrlmod: bool,
    pub mouse_pressed: bool,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            paused: true,
            shiftmod: false,
            ctrlmod: false,
            mouse_pressed: false,
        }
    }
}

impl Controller {
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}
