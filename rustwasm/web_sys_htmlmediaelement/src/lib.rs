use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use gloo::{events::EventListener};
/*
How to play a video with HtmlMediaElement?

This example demonstrates the use of the play method of web-sys HtmlMediaElement. 
A HTML button is first created and associated with a button click event. 
When the button is clicked, the play method of the HTML video element is executed.
The play method returns a promise that is wrapped in a JsFuture.

https://www.webassemblyman.com/rustwasm/web_sys_htmlmediaelement_play.html
*/

pub fn web_sys_htmlmediaelement_play(){
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let button = document.create_element("button")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlButtonElement>()
                    .unwrap();
    button.set_text_content(Some("Play"));

    let paragraph = document.get_element_by_id("message")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlParagraphElement>()
                    .unwrap();
                        

    let on_click = EventListener::new(&button, "click", move |_event| {
        let _video = document.get_element_by_id("my-video")
        .unwrap()
        .dyn_into::<web_sys::HtmlMediaElement>()
        .unwrap();
    
        let promise=_video.play().unwrap();
        
        let _future = async{
            let _result= wasm_bindgen_futures::JsFuture::from(promise).await;
        };
        paragraph.set_text_content(Some("Playing..."));
    
    });

    on_click.forget();     
    body.append_child(&button).unwrap();
}

/*
How to pause a video with HtmlMediaElement?

This example demonstrates the use of the pause method of HTMLMediaElement Struct.
The example starts by creating a button and associatit with a mouse click event. When the button is clicked,
the video element is retrieved using the get_element_by_id method. 
The synchronous pause method is then called to pause the video.

https://www.webassemblyman.com/rustwasm/web_sys_htmlmediaelement_pause.html
*/

pub fn web_sys_htmlmediaelement_pause(){
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    let button = document.create_element("button")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlButtonElement>()
                    .unwrap();
    button.set_text_content(Some("Pause"));

    let paragraph = document.get_element_by_id("message")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlParagraphElement>()
                    .unwrap();

    let on_click = EventListener::new(&button, "click", move |_event| {
        let _video = document.get_element_by_id("my-video")
        .unwrap()
        .dyn_into::<web_sys::HtmlMediaElement>()
        .unwrap();
    
        let _result=_video.pause();
        
        paragraph.set_text_content(Some("Pausing..."));
    
    });

    on_click.forget();     
    body.append_child(&button).unwrap();
}


#[wasm_bindgen(start)]
pub async fn start() {
    web_sys_htmlmediaelement_play();
    web_sys_htmlmediaelement_pause();
  
}