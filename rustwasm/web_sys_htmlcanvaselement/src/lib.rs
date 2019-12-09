use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

//Get the drawing context on the canvas.
pub fn htmlcanvaselement_get_context()
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    context
        .rect(75.0, 75.0, 50.0, 50.0);

    context.stroke();

}

#[wasm_bindgen(start)]
pub fn start() {
    htmlcanvaselement_get_context();
}