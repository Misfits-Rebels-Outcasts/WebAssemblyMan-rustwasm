extern crate wasm_bindgen; 
extern crate js_sys;
use wasm_bindgen::prelude::*; 

#[wasm_bindgen]
pub fn array_new() {
    let arr = js_sys::Array::new();
    arr.push(&"Item 1".into());
    arr.push(&"Item 2".into());
    arr.push(&"Item 3".into());
    web_sys::console::log(&arr);
}

#[wasm_bindgen]
pub fn array_from() {
    let src_arr = js_sys::Array::new();
    src_arr.push(&"Cat A".into());
    src_arr.push(&"Cat B".into());
    let target_arr =js_sys::Array::from(&src_arr);
    web_sys::console::log(&target_arr);
    web_sys::console::log_2(&"Array Length: %d".into(),&JsValue::from(target_arr.length()));
}

#[wasm_bindgen]
pub fn array_set() {
    web_sys::console::log_1(&"Start".into());
    let arr = js_sys::Array::new_with_length(3);
    arr.set(0,JsValue::from_f64(1.0));    
    arr.set(1,JsValue::from_f64(2.0));    
    arr.set(2,JsValue::from_f64(3.0));    
    web_sys::console::log(&arr);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    array_new();
    array_from();
    array_set();
    Ok(())

}

