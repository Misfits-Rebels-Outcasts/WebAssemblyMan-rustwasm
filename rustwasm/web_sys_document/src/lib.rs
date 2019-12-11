use wasm_bindgen::prelude::*;

//Create a paragraph element and append it to the HTML body.
pub fn document_create_element()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let val = document.create_element("p").unwrap();
    val.set_inner_html("Hello World from WebAssemblyMan!");
    body.append_child(&val).unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    document_create_element();
}
/*
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

    Ok(())
    
}
*/