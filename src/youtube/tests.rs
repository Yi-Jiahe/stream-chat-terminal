use http::response;
use std::cell::RefCell;

use super::client::{Client, HttpSend};
use super::structs::*;

pub struct MockSender(RefCell<response::Builder>, &'static str);
impl HttpSend for MockSender {
    fn send(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let mut builder = self.0.borrow_mut();
        let response = builder.body(self.1)?;
        Ok(response.into())
    }
}

#[test]
fn get_widget() {
    let mut builder = response::Builder::new();
    builder.status(200);
    let body = r#"{
       "id": 42,
       "foo": "bar",
       "baz": "quux"
    }"#;
    let sender = MockSender(RefCell::new(builder), body);
    let client = Client::with_sender(sender, Some("".to_string()));

    // let result = client.get_widget("42")
    //    .expect("get_widget() call did not succeed");

    // assert_eq!(
    //    result,
    //    json!({
    //       "id": 42,
    //       "foo": "bar",
    //       "baz": "quux"
    //    })
    // );
}
