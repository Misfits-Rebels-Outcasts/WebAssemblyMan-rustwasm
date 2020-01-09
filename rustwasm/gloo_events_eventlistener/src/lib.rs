use gloo::{events::EventListener};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

//Create a button element with an EventListener. When the button is clicked, a message will be logged to the console.
pub fn eventlistener_new_button_click()
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

//Create a paragraph element with an EventListener. When the mousedown event is triggered, a message will be logged to the console.
pub fn eventlistener_new_p_mousedown()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");

    let paragraph = document.create_element("p").unwrap()
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();

    paragraph.set_align("center");
    paragraph.set_inner_html("<br />Click within this boundary to test the mousedown event. <br />Check the results in your web console.<br /><br />");

    paragraph.style()
        .set_property("border", "solid")
        .map_err(|_| ())
        .unwrap();

    let on_down = EventListener::new(&paragraph, "mousedown", move |_event| {
        web_sys::console::log_1(&"Paragrapah mousedown".into());
    });
    on_down.forget();
    body.append_child(&paragraph).unwrap();

}

//Create a paragraph element with an EventListener. When the mousemove event is triggered, a message will be logged to the console.
pub fn eventlistener_new_p_mousemove()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");

    let paragraph = document.create_element("p").unwrap()
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();

    paragraph.set_align("center");
    paragraph.set_inner_html("<br />Move within this boundary to test the mousemove event. <br />Check the results in your web console.<br /><br />");

    paragraph.style()
        .set_property("border", "solid")
        .map_err(|_| ())
        .unwrap();

    let on_move = EventListener::new(&paragraph, "mousemove", move |_event| {
        web_sys::console::log_1(&"Paragrapah mousemove".into());
    });
    on_move.forget();
    body.append_child(&paragraph).unwrap();

}

//Create a paragraph element with an EventListener. When the mouseup event is triggered, a message will be logged to the console.
pub fn eventlistener_new_p_mouseup()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");

    let paragraph = document.create_element("p").unwrap()
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();

    paragraph.set_align("center");
    paragraph.set_inner_html("<br />Move within this boundary to test the mouseup event. <br />Check the results in your web console.<br /><br />");

    paragraph.style()
        .set_property("border", "solid")
        .map_err(|_| ())
        .unwrap();

    let on_up = EventListener::new(&paragraph, "mouseup", move |_event| {
        web_sys::console::log_1(&"Paragrapah mouseup".into());
    });
    on_up.forget();
    body.append_child(&paragraph).unwrap();

}

/*
This example creates a keydown event using the EventListener Struct and bind it with a HTML textarea element. 
Whenever a key is entered with the keyboard on the textarea, the keyboard event type and key is set as the text content of a paragraph element.
*/
pub fn eventlistener_keyboardevent_keydown(){
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");

    let text_area = document.get_element_by_id("text_area")
                            .unwrap()
                            .dyn_into::<web_sys::HtmlTextAreaElement>()
                            .unwrap();

    let message = document.get_element_by_id("message")
                            .unwrap()
                            .dyn_into::<web_sys::HtmlParagraphElement>()
                            .unwrap();

    let on_keydown = EventListener::new(&text_area, "keydown", move |event| {

            let keyboard_event = event.clone()
                                .dyn_into::<web_sys::KeyboardEvent>()
                                .unwrap();

            let mut event_string = String::from("");
            event_string.push_str(&event.type_());
            event_string.push_str(&" : ");
            event_string.push_str(&keyboard_event.key());
            
            message.set_text_content(Some(&event_string));
    
    });

    on_keydown.forget();     
    
}


#[wasm_bindgen(start)]
pub fn start() {
    //eventlistener_new_button_click();
    //eventlistener_new_p_mousedown();
    //eventlistener_new_p_mousemove();
    //eventlistener_new_p_mouseup();
    eventlistener_keyboardevent_keydown();
}