use wasm_bindgen::prelude::*;
use web_sys::{FontFace};

/*
This example uses the new_with_str method to create a new instance of FontFace. 
The method takes in two parameters, a name that will be used as the font face value for font-family properties, and a 
second parameter, the font source, which can be a URL or binary font data. 
In our scenario below, both a URL to a True Type font and an Open Type font are demonstrated.
*/
pub fn web_sys_fontface_new_with_str() 
{
    let window = web_sys::window().expect("global window does not exists");    
    let _document = window.document().expect("expecting a document on window");
    let _fontface_ttf = FontFace::new_with_str("barcodefont", "url(ConnectCode39.ttf)").unwrap();
    let _fontface_otf = FontFace::new_with_str("barcodefont", "url(ConnectCode39.otf)").unwrap();
    
    /*
    let promise_ttf=fontface_ttf.load().unwrap();
    let promise_otf=fontface_otf.load().unwrap();

    let result= wasm_bindgen_futures::JsFuture::from(promise_ttf).await;
    let result= wasm_bindgen_futures::JsFuture::from(promise_otf).await;

    document.fonts().add(&fontface_ttf);    
    document.fonts().add(&fontface_otf);    
    */

}

pub async fn web_sys_fontface_load() 
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let fontface = FontFace::new_with_str("barcodefont", "url(ConnectCode39.ttf)").unwrap();
    let promise=fontface.load().unwrap();
    let _result= wasm_bindgen_futures::JsFuture::from(promise).await;
    document.fonts().add(&fontface).unwrap();
}

#[wasm_bindgen(start)]
pub async fn start() {
    //web_sys_fontface_new_with_str();
    web_sys_fontface_load().await;

}