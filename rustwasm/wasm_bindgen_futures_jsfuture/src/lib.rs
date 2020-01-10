use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::Response;
use web_sys::Blob;
use web_sys::Url;
use web_sys::HtmlImageElement;

/*
Many web APIs works asynchornously using Promise. Rust on the other hand uses JsFuture for handling asychronous operations elegantly.
This example demonstrates wrapping a JavaScript Promise into a Rust JsFuture. Basically, a PNG image is retrieved using the window.fetch_with_request method.
The response is then placed into an image element in the DOM.

The use of JsFuture await is shown below. In the example, resp_value is getting a value from the fetch_with_request method asynchronously. 
During "await" for the value, other tasks are allowed to take over the current thread.
When the value is finally available, the function will then proceed to the next line of code to continue its execution.
*/

pub async fn wasm_bindgen_future_jsfuture_from()
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let image:HtmlImageElement = document
                    .get_element_by_id("my-image").unwrap().dyn_into().unwrap();    
    let request = Request::new_with_str("/webassembly.png").unwrap();

    //Wrap Promise into JsFuture
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    let response:Response = resp_value.unwrap().dyn_into().unwrap();

    //Wrap Promise into JsFuture
    let blob_value = JsFuture::from(response.blob().unwrap()).await;
    let blob:Blob = blob_value.unwrap().dyn_into().unwrap();
    let object_url = Url::create_object_url_with_blob(&blob);
    image.set_src(&object_url.unwrap());

}

#[wasm_bindgen(start)]
pub async fn start() {
    wasm_bindgen_future_jsfuture_from().await;
}
