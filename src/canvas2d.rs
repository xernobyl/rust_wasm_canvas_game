//use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

use std::cell::RefCell;
use std::rc::Rc;

use web_sys::*;


#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log(s: &str);
}


pub struct Canvas2D {
	canvas: Rc<HtmlCanvasElement>,
	context: Rc<CanvasRenderingContext2d>,
	key_callback: Fn,
	frame_callback: Fn,
	resize_callback: Fn,
}


impl Canvas2D {
	pub fn new() -> Result<Self, String> where Self: Sized {
		fn create_canvas_element() -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
			let document = window().ok_or("Can't get window")?
				.document().ok_or("Can't get document")?;

			let canvas = document
				.create_element("canvas")?
				.dyn_into::<HtmlCanvasElement>()?;

			canvas.style().set_property("position", "fixed")?;
			canvas.style().set_property("left", "0")?;
			canvas.style().set_property("top", "0")?;
			canvas.style().set_property("width", "100%")?;
			canvas.style().set_property("height", "100%")?;

			document.body().unwrap().append_child(&canvas)?;

			let width = canvas.client_width() as u32;
			let height = canvas.client_height() as u32;

			if width != 0 && height != 0 {
				canvas.set_width(width);
				canvas.set_height(height);
			}

			let context = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;

			Result::Ok((canvas, context))
		}

		fn js_to_str(value: JsValue) -> String {
			value.as_string().unwrap_or_else(|| "???".to_string())
		}

		let (canvas, context) = create_canvas_element().map_err(js_to_str)?;
		let canvas = Rc::new(canvas);
		let context = Rc::new(context);

		// keydown event
		{
			let closure = Closure::wrap(Box::new(move |keyboard_event: web_sys::KeyboardEvent| {
				// game.key_pressed(keyboard_event.code(), true);
			}) as Box<dyn FnMut(_)>);

			window().unwrap().set_onkeydown(Option::Some(closure.as_ref().unchecked_ref()));
			closure.forget();
    }

		// keyup event
		{
			let closure = Closure::wrap(Box::new(move |keyboard_event: web_sys::KeyboardEvent| {
				// game.key_pressed(keyboard_event.code(), false);
			}) as Box<dyn FnMut(_)>);

			window().unwrap().set_onkeyup(Option::Some(closure.as_ref().unchecked_ref()));
			closure.forget();
    }

		Result::Ok(Self {
			canvas,
			context,
			key_callback: |_, _| {},
			frame_callback: |_| {},
			resize_callback: |_, _| {},
		})
	}

	pub fn start_loop(self) {
		fn request_animation_frame(f: &Closure<FnMut(f64)>) {
			window().unwrap()
				.request_animation_frame(f.as_ref().unchecked_ref())
				.expect("no requestAnimationFrame");
		}

		log(format!("Starting loop...").as_ref());

		let mut rc = Rc::new(self);
		let f = Rc::new(RefCell::new(None));
		let g = f.clone();

		let closure = Some(Closure::wrap(Box::new(move |timestamp| {
			if let Some(the_self) = Rc::get_mut(&mut rc) {
				self.frame_callback(timestamp);
			};
			request_animation_frame(f.borrow().as_ref().unwrap());
		}) as Box<dyn FnMut(_)>));

		*g.borrow_mut() = closure;
		request_animation_frame(g.borrow().as_ref().unwrap());
		// closure.forget();
	}

	pub fn on_key(&mut self, callback: Fn) {
		self.key_callback = callback;
	}

	pub fn on_frame(&mut self, callback: Fn) {
		self.frame_callback = callback;
	}

	pub fn on_resize(&mut self, callback: Fn) {
		self.resize_callback = callback;
	}

	pub fn log(&self, text: &str) {
		log(text);
	}
}
