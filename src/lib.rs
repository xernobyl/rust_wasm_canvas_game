use wasm_bindgen::prelude::*;

pub mod canvas2d;
pub mod game;
pub mod player;

use self::canvas2d::Canvas2D;

struct Game {
	canvas: Canvas2D,
}

impl Game {
	fn main() -> Result<(), JsValue> {
		let canvas = Canvas2D::new()?;

		canvas.on_key(|key_code, down| {
			canvas.log("FUCKING KEY PRESSED!");
		});

		canvas.on_frame(|frame_time| {
			canvas.log("FUCKING NEW FRAME!");
		});

		canvas.on_resize(|new_width, new_height| {
			canvas.log("FUCKING RESIZED!");
		});

		canvas.start_loop();

		Ok(())
	}
}


#[wasm_bindgen(start)]
pub fn web_main() -> Result<(), JsValue> {
	Game::main()
}
