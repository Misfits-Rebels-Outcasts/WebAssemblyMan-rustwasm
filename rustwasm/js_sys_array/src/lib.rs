extern crate wasm_bindgen; 
extern crate js_sys;
use wasm_bindgen::prelude::*; 

//Create a js_sys Array with the new function and push 3 items into the Array.
pub fn array_new() {
    let arr = js_sys::Array::new();
    arr.push(&"Item 1".into());
    arr.push(&"Item 2".into());
    arr.push(&"Item 3".into());
    web_sys::console::log(&arr);
}

//Create a new js_sys Array with a specified length of 3. 
pub fn array_new_with_length() {
    let arr = js_sys::Array::new_with_length(3);
    arr.set(0,JsValue::from_f64(1.0));    
    arr.set(1,JsValue::from_f64(2.0));    
    arr.set(2,JsValue::from_f64(3.0));    
    web_sys::console::log(&arr);
}

//Create a new Array from an existing Array.
pub fn array_from() {
    let src_arr = js_sys::Array::new();
    src_arr.push(&"Cat A".into());
    src_arr.push(&"Cat B".into());
    let target_arr =js_sys::Array::from(&src_arr);
    web_sys::console::log(&target_arr);
    web_sys::console::log_2(&"Array Length: %d".into(),&JsValue::from(target_arr.length()));
}

//Set the cells of an Array with a JsValue Struct.
pub fn array_set() {
    let arr = js_sys::Array::new_with_length(3);
    arr.set(0,JsValue::from_f64(1.0));    
    arr.set(1,JsValue::from_f64(2.0));    
    arr.set(2,JsValue::from_f64(3.0));    
    web_sys::console::log(&arr);
}

//Get the cell located at index 1 and print it to the console.
pub fn array_get() {
    let arr = js_sys::Array::new_with_length(3);
    arr.set(0,JsValue::from_str("Apple"));    
    arr.set(1,JsValue::from_str("Orange"));    
    web_sys::console::log_2(&"Get Value:%s".into(),&arr.get(1));
}

//Concat the Arrays arr1 and arr2 to create an arr3 Array.
pub fn array_concat() {
    let arr1 =js_sys::Array::new_with_length(1);
    arr1.set(0,JsValue::from_str("Apple"));    
    let arr2 =js_sys::Array::new_with_length(1);
    arr2.set(0,JsValue::from_str("Orange"));    
    let arr3=arr1.concat(&arr2);
    web_sys::console::log(&arr3);
}

//Loop an Array and print each of the items to the console.
pub fn array_for_each() {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from("Apple"));
    arr.push(&JsValue::from("Orange"));    
    arr.for_each(&mut |obj, idx, _arr| 
        web_sys::console::log_3(&"Array Object %d: %s ".into(),&JsValue::from(idx),&obj)
    );
}

//Find the item "Jane" in an Array and print it to the console log.
pub fn array_find() {
    
    let arr= js_sys::Array::new();
    arr.push(&JsValue::from("John"));
    arr.push(&JsValue::from("Jane"));    
    arr.push(&JsValue::from("James"));    

    let find_item=arr.find(&mut |obj, idx, _arr|     
        JsValue::as_string(&obj).unwrap()=="Jane"   
    );    
    web_sys::console::log_2(&"Item found: %s".into(),&find_item);
    
}

//Find the index of the item "Jane" located in an Array and print the index of the item found.
pub fn array_find_index() {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from("John"));
    arr.push(&JsValue::from("Jane"));    
    arr.push(&JsValue::from("James"));    
    let find_index=arr.find_index(&mut |obj, idx, _arr|     
        JsValue::as_string(&obj).unwrap()=="Jane"   
    );    
    web_sys::console::log_2(&"Index found : %s ".into(),&JsValue::from(find_index));
}

//Delete an item located at position 1 in an Array.
pub fn array_delete() {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from("John"));
    arr.push(&JsValue::from("Jane"));    
    arr.push(&JsValue::from("James"));    
    web_sys::console::log_2(&"Array Length: %d".into(),&JsValue::from(arr.length()));
    arr.delete(1);
    web_sys::console::log_2(&"Array Length: %d".into(),&JsValue::from(arr.length()));

    web_sys::console::log_2(&"Array Item 1: %s".into(),&arr.get(0));
    web_sys::console::log_2(&"Array Item 2: %s".into(),&arr.get(1));
    web_sys::console::log_2(&"Array Item 3: %s".into(),&arr.get(2));

}

//Push items into an Array and print the length of the Array.
pub fn array_push() {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from("John"));
    web_sys::console::log_2(&"Array Length: %d".into(),
                            &JsValue::from(arr.push(&JsValue::from("Jane")))
                            );
    web_sys::console::log_2(&"Array Length: %d".into(),
                            &JsValue::from(arr.push(&JsValue::from("James")))
                            );
}

//Find an item "Jane" from position 0. The Array will be search from the beginning to the end.
pub fn array_index_of() {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from("John"));
    arr.push(&JsValue::from("Jane"));    
    let idx=arr.index_of(&JsValue::from("Jane"),0);   
    web_sys::console::log_2(&"Index Found : %d ".into(),&JsValue::from(idx));
}

//Find the item "Jane" backwards from position 1.
pub fn array_last_index_of() {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from("Jane"));    
    arr.push(&JsValue::from("John"));
    arr.push(&JsValue::from("Jane"));    
    arr.push(&JsValue::from("James"));
    let idx=arr.last_index_of(&JsValue::from("Jane"),1);   
    web_sys::console::log_2(&"Index Found : %d ".into(),&JsValue::from(idx));
    //Index Found : 0
}

//Fill the Array with "Init Value" for position 0 to 4
pub fn array_fill() {
    let arr = js_sys::Array::new_with_length(5);
    let target_arr=arr.fill(&JsValue::from("Init Value"),0,5); 
    //let target_arr=arr.fill(&JsValue::from_f64(5.0),0,5);       
    web_sys::console::log_2(&"Array Length: %d".into(),
                            &JsValue::from(target_arr.length())
                            );
    web_sys::console::log_2(&"Item 1: %s".into(),
                            &JsValue::from(target_arr.get(0))
                            );
}

/*

    let a12 = js_sys::Array::new();
    a12.push(&JsValue::from("John"));
    a12.push(&JsValue::from("Jane"));    
    let zincludes=a12.includes(&JsValue::from("Jane"),0);   
    web_sys::console::log_2(&"zincludes ".into(),&JsValue::from(zincludes));

pub fn entries(&self) -> Iterator
The entries() method returns a new Array Iterator object that contains the key/value pairs for each index in the array.

pub fn filter(
    &self, 
    predicate: &mut dyn FnMut(JsValue, u32, Array) -> bool
) -> Array
The filter() method creates a new array with all elements that pass the test implemented by the provided function.

pub fn flat(&self, depth: i32) -> Array
The flat() method creates a new array with all sub-array elements concatenated into it recursively up to the specified depth.

pub fn flat_map(
    &self, 
    callback: &mut dyn FnMut(JsValue, u32, Array) -> Vec<JsValue>
) -> Array
The flatMap() method first maps each element using a mapping function, then flattens the result into a new array.

pub fn join(&self, delimiter: &str) -> JsString
The join() method joins all elements of an array (or an array-like object) into a string and returns this string.

pub fn map(
    &self, 
    predicate: &mut dyn FnMut(JsValue, u32, Array) -> JsValue
) -> Array
map calls a provided callback function once for each element in an array, in order, and constructs a new array from the results. callback is invoked only for indexes of the array which have assigned values, including undefined. It is not called for missing elements of the array (that is, indexes that have never been set, which have been deleted or which have never been assigned a value).

pub fn of1(a: &JsValue) -> Array
The Array.of() method creates a new Array instance with a variable number of arguments, regardless of number or type of the arguments.
The difference between Array.of() and the Array constructor is in the handling of integer arguments: Array.of(7) creates an array with a single element, 7, whereas Array(7) creates an empty array with a length property of 7 (Note: this implies an array of 7 empty slots, not slots with actual undefined values).

of2..of5

pop
pub fn pop(&self) -> JsValue
The pop() method removes the last element from an array and returns that element. This method changes the length of the array.

reduce
pub fn reduce(
    &self, 
    predicate: &mut dyn FnMut(JsValue, JsValue, u32, Array) -> JsValue, 
    initial_value: &JsValue
) -> JsValue
The reduce() method applies a function against an accumulator and each element in the array (from left to right) to reduce it to a single value.

reduce_right
pub fn reduce_right(
    &self, 
    predicate: &mut dyn FnMut(JsValue, JsValue, u32, Array) -> JsValue, 
    initial_value: &JsValue
) -> JsValue
The reduceRight() method applies a function against an accumulator and each value of the array (from right-to-left) to reduce it to a single value.

reverse
pub fn reverse(&self) -> Array
The reverse() method reverses an array in place. The first array element becomes the last, and the last array element becomes the first.

shift
pub fn shift(&self) -> JsValue
The shift() method removes the first element from an array and returns that removed element. This method changes the length of the array.

slice
pub fn slice(&self, start: u32, end: u32) -> Array
The slice() method returns a shallow copy of a portion of an array into a new array object selected from begin to end (end not included). The original array will not be modified.

some
pub fn some(&self, predicate: &mut dyn FnMut(JsValue) -> bool) -> bool
The some() method tests whether at least one element in the array passes the test implemented by the provided function. Note: This method returns false for any condition put on an empty array. MDN documentation

sort
pub fn sort(&self) -> Array
The sort() method sorts the elements of an array in place and returns the array. The sort is not necessarily stable. The default sort order is according to string Unicode code points.
The time and space complexity of the sort cannot be guaranteed as it is implementation dependent.

splice
pub fn splice(&self, start: u32, delete_count: u32, item: &JsValue) -> Array
The splice() method changes the contents of an array by removing existing elements and/or adding new elements.

to_locale_string
pub fn to_locale_string(&self, locales: &JsValue, options: &JsValue) -> JsString
The toLocaleString() method returns a string representing the elements of the array. The elements are converted to Strings using their toLocaleString methods and these Strings are separated by a locale-specific String (such as a comma “,”).

to_string
pub fn to_string(&self) -> JsString
The toString() method returns a string representing the specified array and its elements.

unshift
pub fn unshift(&self, value: &JsValue) -> u32
The unshift() method adds one or more elements to the beginning of an array and returns the new length of the array.

pub fn keys(&self) -> Iterator
The keys() method returns a new Array Iterator object that contains the keys for each index in the array.

pub fn entries(&self) -> Iterator
The entries() method returns a new Array Iterator object that contains the key/value pairs for each index in the array.

pub fn values(&self) -> Iterator
The values() method returns a new Array Iterator object that contains the values for each index in the array.
*/

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    array_new();
    /*
    array_new_with_length();
    array_from();
    array_set();
    array_get();
    array_delete();
    array_push();
    array_fill();
    array_index_of();
    array_last_index_of();
    array_concat();
    array_for_each();
    array_find();
    array_find_index();
    */
    Ok(())

}

