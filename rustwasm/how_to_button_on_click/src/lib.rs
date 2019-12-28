use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use gloo::{events::EventListener};

/*
How to add an onclick event to a button using gloo?

This example demonstrates the use of create_element to create a button element. 
After which, the gloo EventListener is used to add an onclick event to the button.
When the button is clicked, the function is triggered to add a paragraph element with the text content "Hello World".

https://www.webassemblyman.com/rustwasm/button_onclick.html
*/
pub fn how_to_button_on_click_gloo(){
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");

    let button = document.create_element("button")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlButtonElement>()
                    .unwrap();
    button.set_text_content(Some("Gloo : Say Hello"));

    let paragraph =document.create_element("p")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlParagraphElement>()
                    .unwrap();                    
    body.append_child(&paragraph).unwrap();
    /*
    let paragraph1= document
        .get_element_by_id("message")
        .unwrap()
        .dyn_into::<web_sys::HtmlParagraphElement>()
        .unwrap();
    let paragraph1_hello = paragraph1.clone();
    */

    let on_click = EventListener::new(&button, "click", move |_event| {
        web_sys::console::log_2(&"Hello World Gloo :%s".into(),&"WebAssemblyMan".into());
        paragraph.set_text_content(Some("Gloo: Hello World"));
        //paragraph1_hello.set_text_content(Some("Hello World"));
    
    });

    on_click.forget();     
    body.append_child(&button).unwrap();
    //body.append_child(&paragraph1).unwrap();
}

/*
How to add an onclick event to a button?

This example demonstrates the use of get_element_by_id to get a button element from the DOM. 
After which, Closures, functions that can capture the enclosing environment, is used to add an onclick event to the button.
When the button is clicked, the function is triggered to change the text content of the paragraph element to "Hello World".

https://www.webassemblyman.com/rustwasm/button_onclick.html
*/
pub fn how_to_button_on_click(){
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");

    let document_hello=document.clone();

    let say_hello = Closure::wrap(Box::new(move || {
        web_sys::console::log_2(&"Hello World Closures :%s".into(),&"WebAssemblyMan".into());
        document_hello
            .get_element_by_id("message")
            .expect("should have a button on the page")
            .dyn_ref::<web_sys::HtmlElement>()
            .expect("#message be an `HtmlElement`")
            .set_text_content(Some("Closures: Hello World"));

    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("button-click-test")
        .expect("should have a button on the page")
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("#button-click-test be an `HtmlElement`")
        .set_onclick(Some(say_hello.as_ref().unchecked_ref()));
    
    say_hello.forget();

}

#[wasm_bindgen(start)]
pub fn start() {
    how_to_button_on_click();
    how_to_button_on_click_gloo();
  
}