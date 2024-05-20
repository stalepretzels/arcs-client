mod model;
use js_sys::Array;
use model::*;

use wasm_bindgen::prelude::*;
use web_sys::{ window, Element };
use urlencoding::encode;
use chrono::{DateTime,Local};
extern crate console_error_panic_hook;
use std::panic;

pub trait AppendMessage {
    fn append_message(&self, msg: &str) -> ();
}

impl AppendMessage for web_sys::Document {
    fn append_message(&self, msg: &str) -> () {
        let elem = self.create_element("p").unwrap();
        elem.set_text_content(Some(msg));
        self.body().unwrap().append_child(&elem).unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub async fn synthesize(data: &str) -> Array {
    let arr = Array::new();
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    match serde_json::from_str::<MessageTypes>(data).unwrap() {
        MessageTypes::MessageSent(response) => {
            arr.push(&create_message_element(response));
            arr
        }
        MessageTypes::RetrieveMessages(response) => {
            let iterator = response.msgs.iter();
            for (_i, d) in iterator.clone().enumerate() {
                arr.push(&create_message_element(d.clone()));
            }
            arr
        }
        MessageTypes::UserJoin(response) => {
            let message = document.create_element("div").unwrap();
            message.set_class_name("statusMsg");
            message.set_text_content(
                Some(&format!("{} joined this chat room.", response.userjoin))
            );
            arr.push(&message);
            arr
        }
        MessageTypes::UserLeft(response) => {
            let message = document.create_element("div").unwrap();
            message.set_class_name("statusMsg");
            message.set_text_content(Some(&format!("{} left this chat room.", response.userleft)));
            arr.push(&message);
            arr
        }
    }
}

pub fn create_message_element(res: MessageSent) -> Element {
    // Get the document and create a new div element
    let document = window().unwrap().document().unwrap();
    let outer_div = document.create_element("div").unwrap();
    outer_div.set_attribute("style", "margin: 10px 0;").unwrap();

    // Create the inner div with the class "userDisplay"
    let user_display_div = document.create_element("div").unwrap();
    user_display_div.set_class_name("userDisplay");

    // Create and append the jdenticon element
    /*let jdenticon_svg = document.create_element("svg").unwrap();
    jdenticon_svg.set_inner_html(&format!("{}", jdenticon.to_svg(username[2], 26))); We'll wait for this...
    user_display_div.append_child(&jdenticon_svg).unwrap();*/

    // Create and append the anchor element
    let anchor = document.create_element("a").unwrap();
    anchor.set_attribute("href", &format!("/profile/{}", encode(&res.user))).unwrap();
    anchor.set_text_content(Some(&res.user));
    user_display_div.append_child(&anchor).unwrap();

    // Create and append the span element with class "msgTime"
    let time_span = document.create_element("span").unwrap();
    time_span.set_class_name("msgTime");
    time_span.set_inner_html(
        &format!(
            "[at <time datetime='{}'>{}</time>]",
            res.time.unwrap(),
            DateTime::<Local>::from(res.time.unwrap()).format("%x %r")
        )
    );
    user_display_div.append_child(&time_span).unwrap();

    outer_div.append_child(&user_display_div).unwrap();

    // Create the message div with id "message" and a pre element inside
    let message_div = document.create_element("div").unwrap();
    message_div.set_id("message");

    let pre_element = document.create_element("pre").unwrap();
    pre_element.set_text_content(Some(&res.msg));

    message_div.append_child(&pre_element).unwrap();

    outer_div.append_child(&message_div).unwrap();

    return outer_div;
}

#[wasm_bindgen]
pub fn input_to_message(input: Array) -> JsValue {
    let msg = MessageSent {
        msg: input.get(0).as_string().unwrap(),
        user: input.get(1).as_string().unwrap(),
        time: None,
    };

    return JsValue::from_str(&serde_json::to_string(&msg).expect("failed to serialize"));
}
