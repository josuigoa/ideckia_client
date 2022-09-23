use crate::GUI;
use std::thread;
use ws::{connect, Handler, Handshake, Message, Result, Sender};

pub struct WebSocketClient {
    out: Sender,
}

impl WebSocketClient {
    pub fn connect() {
        thread::spawn(move || {
            connect("ws://localhost:8888", |out| WebSocketClient { out }).unwrap()
        });
    }
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for WebSocketClient {
    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        GUI::set_sender(&self.out);
        Result::Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let parsed: serde_json::Value =
            serde_json::from_str(&msg.as_text().unwrap()).expect("Can't parse to JSON");

        match parsed.get("type") {
            Some(json_type) => match json_type.as_str() {
                Some(d) => {
                    if d == "layout" {
                        match parsed.get("data") {
                            Some(data) => GUI::set_layout(data),
                            None => println!("No data received from server"),
                        }
                    }
                }
                None => {}
            },
            None => {}
        }
        Result::Ok(())
    }
}
