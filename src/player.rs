pub struct Player {
	position: (f32, f32),
}


impl Player {
	pub fn new() -> Self {
		Self {
			position: (0.0, 0.0),
		}
	}
}
