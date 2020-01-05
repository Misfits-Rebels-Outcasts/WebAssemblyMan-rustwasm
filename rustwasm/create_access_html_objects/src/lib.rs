use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn create_htmlcanvaselement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _canvas = document.create_element("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .unwrap();

}

pub fn access_htmlcanvaselement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _canvas = document.get_element_by_id("my-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

}

#[wasm_bindgen(start)]
pub fn start() {
    create_htmlcanvaselement();
    access_htmlcanvaselement();
}