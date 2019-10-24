extern crate wasm_bindgen; 
extern crate js_sys;
use wasm_bindgen::prelude::*; 

//assert
pub fn console_assert() {
    web_sys::console::assert();     
}

//"Assertion Failed" will be printed to the console for the condition z<y.
pub fn console_assert_with_condition_and_data() {
    let x=5u32;
    let y=10u32;   
    let z=15u32; 
    let array = js_sys::Array::new();
    array.push(&"Assertion Failed".into());
    //Assertion passed                                
    web_sys::console::assert_with_condition_and_data(x<y,&array);
    //Assertion failed                                
    web_sys::console::assert_with_condition_and_data(z<y,&array); 
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    //console_assert();
    console_assert_with_condition_and_data();
    Ok(())

}

