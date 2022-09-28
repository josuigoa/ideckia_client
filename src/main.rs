use image::io::Reader as ImageReader;
use std::io::Cursor;

fn main() {
    tracing_subscriber::fmt::init();

    ideckia_client::websocket_client::WebSocketClient::connect();

    let mut native_options = eframe::NativeOptions {
        ..Default::default()
    };

    native_options.icon_data = get_icon_data();

    eframe::run_native(
        "ideckia client",
        native_options,
        Box::new(|cc| Box::new(ideckia_client::GUI::new(cc))),
    );
}

fn get_icon_data() -> Option<eframe::IconData> {
    let icon_bytes = include_bytes!("../icon.ico");
    let icon = match ImageReader::new(Cursor::new(icon_bytes)).with_guessed_format() {
        Ok(r) => match r.decode() {
            Ok(d) => d,
            Err(e) => {
                println!("Error guessing icon format: {}", e);
                return None;
            }
        },
        Err(e) => {
            println!("Error decoding icon data: {}", e);
            return None;
        }
    };

    let icon = icon.to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    Some(eframe::IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    })
}
