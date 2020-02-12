use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn create_htmlanchorelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _anchor = document.create_element("a")
    .unwrap()
    .dyn_into::<web_sys::HtmlAnchorElement>()
    .unwrap();

}

pub fn access_htmlanchorelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _anchor = document.get_element_by_id("my-anchor")
        .unwrap()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap();
}

pub fn create_htmlbuttonelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _button = document.create_element("button")
    .unwrap()
    .dyn_into::<web_sys::HtmlButtonElement>()
    .unwrap();

}

pub fn access_htmlbuttonelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _button = document.get_element_by_id("my-button")
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()
        .unwrap();

}

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

pub fn create_htmldivelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _div = document.create_element("div")
    .unwrap()
    .dyn_into::<web_sys::HtmlDivElement>()
    .unwrap();

}

pub fn access_htmldivelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _div = document.get_element_by_id("my-div")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();
}

pub fn create_htmlimageelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _img = document.create_element("img")
    .unwrap()
    .dyn_into::<web_sys::HtmlImageElement>()
    .unwrap();

}

pub fn access_htmlimageelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _img = document.get_element_by_id("my-img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
}


pub fn create_htmlpictureelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _picture = document.create_element("picture")
    .unwrap()
    .dyn_into::<web_sys::HtmlPictureElement>()
    .unwrap();

}

pub fn access_htmlpictureelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _picture = document.get_element_by_id("my-picture")
        .unwrap()
        .dyn_into::<web_sys::HtmlPictureElement>()
        .unwrap();
}

pub fn create_htmlparagraphelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _paragraph = document.create_element("p")
    .unwrap()
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .unwrap();

}

pub fn access_htmlparagraphelement()
{

    let document = web_sys::window().unwrap().document().unwrap();      
    let _paragraph = document.get_element_by_id("my-paragraph")
        .unwrap()
        .dyn_into::<web_sys::HtmlParagraphElement>()
        .unwrap();
}


#[wasm_bindgen(start)]
pub fn start() {
    create_htmlanchorelement();
    access_htmlanchorelement();

    //create_htmlbuttonelement();
    //access_htmlbuttonelement();

    //create_htmlcanvaselement();
    //access_htmlcanvaselement();

    //create_htmldivelement();
    //access_htmldivelement();

    //create_htmlimageelement();
    //access_htmlimageelement();

    //create_htmlparagraphelement();
    //access_htmlparagraphelement();

    //create_htmlpictureelement();
    //access_htmlpictureelement();

}