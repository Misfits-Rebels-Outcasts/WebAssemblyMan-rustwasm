use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

//Draw a full circle (2 * pi radian) using the arc function on a HTML canvas
pub fn canvasrenderingcontext2d_arc()
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

    // x - 75 (x-coordinate of the center of the circle)
    // y - 75 (y-coordinate of the center of the circle)
    // r - 50 (radius of the circle)
    // sAngle - 0 (Starting angle in radians) 
    // eAngle - f64::consts::PI * 2.0 (Ending angle in radians)
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

}

//Draw a rectangle using the rect function on a HTML canvas
pub fn canvasrenderingcontext2d_rect()
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

    // x - 75 (x-coordinate of the center of the circle)
    // y - 75 (y-coordinate of the center of the circle)
    // width - the width of the rectangle in pixels
    // height - the height of the rectangle in pixels
    context
        .rect(75.0, 75.0, 50.0, 50.0);
        //.unwrap();

    context.stroke();

}

#[wasm_bindgen(start)]
pub fn start() {
    canvasrenderingcontext2d_arc();
    canvasrenderingcontext2d_rect();
}