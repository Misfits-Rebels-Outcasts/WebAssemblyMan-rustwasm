use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;

//This example calls the window getter to access its document. The URL of the document is then logged to the console. Finally, the new function is called to create a new document. The new document can be associated with the contentDocument of a frame. 
pub fn document_new()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    web_sys::console::log_2(&"URL: %s".into(),&JsValue::from_str(&document.url().unwrap()));
    let new_document = Document::new().unwrap();

}

//This example demonstrates the get_element_by_id method of the document element. The method retrieves the "paragraphId" element and then log its inner text to the console.
pub fn document_get_element_by_id()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    //let body = document.body().expect("document expect to have have a body");
    let val = document.get_element_by_id("paragraphId")
    .unwrap()
    .dyn_into::<web_sys::HtmlElement>()
    .unwrap();
    web_sys::console::log_2(&"URL: %s".into(),&JsValue::from_str(&val.inner_text()));
}

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

//This example creates a div element with a http://www.w3.org/1999/xhtml namespace. The namespace is used to identify which type of XML this tag node belongs to.
//Valid namespace:
//http://www.w3.org/1999/xhtml for HTML
//http://www.w3.org/2000/svg for SVG
pub fn document_create_element_ns()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let val = document.create_element_ns(Some("http://www.w3.org/1999/xhtml"),"div").unwrap();
    val.set_inner_html("Hello World from create_element_ns.");
    body.append_child(&val).unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    document_new();
    //document_create_element();
    //document_get_element_by_id();
    //document_create_element_ns();
}
