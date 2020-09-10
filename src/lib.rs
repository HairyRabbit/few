use dodrio::{Vdom};
use log::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod counter_scss;
mod counter;
use counter::*;

#[wasm_bindgen(start)]
pub fn run() {
    // Initialize debug logging for if/when things go wrong.
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).expect("should initialize logging OK");

    info!("hello world");

    // Get the document's `<body>`.
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let div = document.create_element("div").unwrap();
    body.append_child(&div).unwrap();

    // Construct a new counter component.
    let counter = Counter::new();

    // Mount our counter component to the `<body>`.
    let vdom = Vdom::new(&div, counter);

    // Run the virtual DOM and its listeners forever.
    vdom.forget();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
