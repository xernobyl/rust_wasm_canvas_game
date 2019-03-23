pub struct Game {

}

impl Game {
	pub fn new() -> Self {
		Game {

		}
	}

	fn log(&self, message: &str) {

	}

	pub fn key_pressed(&mut self, key_code: &String, down: &bool) {
		if key_code == "ArrowUp" {
			self.log("UP");
		} else if key_code == "ArrowRight" {
			self.log("RIGHT");
		} else if key_code == "ArrowDown" {
			self.log("DOWN");
		} else if key_code == "ArrowLeft" {
			self.log("LEFT");
		} else {
			self.log(&format!("Key up {}", key_code));
		}
	}
}
