use wasm_bindgen::prelude::*;

pub mod canvas;
pub mod game;
pub mod player;

use self::canvas::Canvas;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn web_main() -> Result<(), JsValue> {
	match Canvas::new() {
		Err(msg) => {
			log(&msg);
			Err(JsValue::from(msg))
		},
		Ok(gl) => {
			gl.start_loop();
			Ok(())
		}
	}
}
