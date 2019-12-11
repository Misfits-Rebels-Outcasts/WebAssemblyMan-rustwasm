use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let val = document.create_element("p")?;
    val.set_inner_html("Hello World from WebAssemblyMan!");
    body.append_child(&val)?;
    Ok(())
    
}
