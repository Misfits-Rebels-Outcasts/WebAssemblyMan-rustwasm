use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::Cell;
use std::rc::Rc;

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

//Set the CSS border and border-radius style of the canvas.
pub fn htmlcanvaselement_style()
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    
    canvas.style().set_property("border", "2px dashed blue")        
        .map_err(|_| ())
        .unwrap();

    canvas.style().set_property("border-radius", "10px")        
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
        .rect(15.0, 15.0, 50.0, 50.0);

    context.fill();

}

//Set the width and height of the canvas.
pub fn htmlcanvaselement_set_width_height()
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(500);
    canvas.set_height(500);
    
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    context
        .rect(75.0, 75.0, 50.0, 50.0);

    context.fill();

}

//Add the mousedown, mousemove and mouseup event to a HTML canvas to implement a paint program.
pub fn htmlcanvaselement_add_event_listener_with_callback()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let canvas = document
        .get_element_by_id("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    
    document.body().unwrap().append_child(&canvas)
        .map_err(|_| ())
        .unwrap();

    
    canvas.set_width(500);
    canvas.set_height(500);
    canvas.style()
        .set_property("border", "inset")
        .map_err(|_| ())
        .unwrap();

    let context_2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|_| ())
        .unwrap();

    let context_2d = Rc::new(context_2d);    
    let down = Rc::new(Cell::new(false));
    //mousedown
    {
        let context_2d = context_2d.clone();
        let down = down.clone();
        let closure = Closure::wrap(Box::new(move |mouse_event: web_sys::MouseEvent| {
            context_2d.begin_path();
            context_2d.move_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
            down.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .map_err(|_| ())
            .unwrap();

        //keep alive
        closure.forget();
    }
    //mousemove
    {
        let context_2d = context_2d.clone();
        let down = down.clone();
        let closure = Closure::wrap(Box::new(move |mouse_event: web_sys::MouseEvent| {
            if down.get() {
                context_2d.line_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
                context_2d.stroke();
                context_2d.begin_path();
                context_2d.move_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .map_err(|_| ())
            .unwrap();

        //keep alive
        closure.forget();
    }
    //mouseup
    {
        let context_2d = context_2d.clone();
        let down = down.clone();
        let closure = Closure::wrap(Box::new(move |mouse_event: web_sys::MouseEvent| {
            down.set(false);
            context_2d.line_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
            context_2d.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .map_err(|_| ())
            .unwrap();

        //keep alive
        closure.forget();
    }
    
    //Ok(())
}

#[wasm_bindgen(start)]
pub fn start() {
    //htmlcanvaselement_get_context();
    //htmlcanvaselement_set_width_height();
    //htmlcanvaselement_style();
    htmlcanvaselement_add_event_listener_with_callback();
}