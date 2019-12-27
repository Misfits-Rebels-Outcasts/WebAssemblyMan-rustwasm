use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

//This example creates an svg element and then use the style getter method to get the CssStyleDeclaration. 
//After which, the set_property method of CssStyleDeclaration is called to set the border property.
//Finally a circle with blue fill and black stroke is added to the svg container.
pub fn svgelement_style()-> Result<(), JsValue>
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
    .unwrap()
    .dyn_into::<web_sys::SvgElement>()
    .map_err(|_| ())
    .unwrap();

    svg
    .style()
    .set_property("border", "solid")?;

    svg.set_attribute("width","500")?;
    svg.set_attribute("height","500")?;
    svg.set_attribute("viewBox", "0 0 500 500")?;

    let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    circle.set_attribute("cx", "100")?;
    circle.set_attribute("cy", "100")?;
    circle.set_attribute("r", "60")?;
    circle.set_attribute("stroke", "black")?;
    circle.set_attribute("fill", "blue")?;
    svg.append_child(&circle)?;

    body.append_child(&svg)?;

    Ok(())
}

#[wasm_bindgen(start)]
pub fn start() {
    svgelement_style().unwrap();

}