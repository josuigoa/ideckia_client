fn main() {
    tracing_subscriber::fmt::init();

    ideckia_client::websocket_client::WebSocketClient::connect();

    let mut native_options = eframe::NativeOptions {
        ..Default::default()
    };

    match image::open("icon.ico") {
        Ok(icon) => {
            let icon = icon.to_rgba8();
            let (icon_width, icon_height) = icon.dimensions();
            native_options.icon_data = Some(eframe::IconData {
                rgba: icon.into_raw(),
                width: icon_width,
                height: icon_height,
            });
        }
        Err(_) => {
            println!("Could not load the icon.");
        }
    }

    eframe::run_native(
        "ideckia client",
        native_options,
        Box::new(|cc| Box::new(ideckia_client::GUI::new(cc))),
    );
}
