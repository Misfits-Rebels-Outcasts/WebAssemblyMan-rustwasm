extern crate wasm_bindgen; 
use wasm_bindgen::prelude::*; 

//Create a JSValue from a string.
pub fn jsvalue_from_str() {
    let js_str=JsValue::from_str(&"Hello World");
    web_sys::console::log_2(&"Get Value:%s".into(),&js_str);
}

pub fn jsvalue_from_f64() {
    let js_f64=JsValue::from_f64(5.0);
    web_sys::console::log_2(&"Get Value:%s".into(),&js_f64);
}
               
pub fn jsvalue_from_bool() {
    let js_bool=JsValue::from_bool(true);
    web_sys::console::log_2(&"Get Value:%s".into(),&js_bool);
}

pub fn jsvalue_undefined() {
    let js_undefined=JsValue::undefined();
    web_sys::console::log_2(&"Get Value:%s".into(),&js_undefined);
}
                
pub fn jsvalue_null() {
    let js_null=JsValue::null();
    web_sys::console::log_2(&"Get Value:%s".into(),&js_null);
}
              
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    jsvalue_from_str();
    jsvalue_from_f64();
    jsvalue_from_bool();
    jsvalue_undefined();
    jsvalue_null();
    Ok(())

}

