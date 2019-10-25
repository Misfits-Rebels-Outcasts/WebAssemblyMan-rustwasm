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

//Log an Array to the console.
pub fn console_log() {
    let array = js_sys::Array::new();
    array.push(&"Hello Console Log".into());
    web_sys::console::log(&array);
}

//Log "Hello World" to the console.
pub fn console_log1() {
     web_sys::console::log_1(&"Hello World".into());
}

//Log "Hello World" with 1 string subsitution to the console.
pub fn console_log2() {
    web_sys::console::log_2(&"%s : Hello World".into(),&"John".into());
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    //console_assert();
    //console_assert_with_condition_and_data();
    console_log();
    console_log1();
    console_log2();
    Ok(())

}

