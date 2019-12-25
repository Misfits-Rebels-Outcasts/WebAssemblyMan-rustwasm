use gloo::{events::EventListener};
use wasm_bindgen::prelude::*;

//Create a button element with an EventListener. When the button is clicked, a message will be logged to the console.
pub fn eventlistener_new()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let button = document.create_element("button").unwrap();
    button.set_inner_html("Button");

    let on_click = EventListener::new(&button, "click", move |_event| {
        web_sys::console::log_1(&"Hello World".into());
    });
    on_click.forget();
    body.append_child(&button).unwrap();

}

#[wasm_bindgen(start)]
pub fn start() {
    eventlistener_new();

}