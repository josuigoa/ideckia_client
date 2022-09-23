use hex_color::HexColor;
use serde_json::Value;

const DEFAULT_TEXT: &str = "";
const DEFAULT_TEXT_SIZE: i64 = 18;
const DEFAULT_TEXT_COLOR: HexColor = HexColor::WHITE;
const DEFAULT_TEXT_POSITION: &str = "bottom";
const DEFAULT_BG_COLOR: HexColor = HexColor::GRAY;

pub struct ItemState {
    pub id: i64,
    pub text: String,
    pub text_size: i64,
    pub text_color: HexColor,
    pub text_position: String,
    pub bg_color: HexColor,
    pub icon_data: String, // nullable
}

impl ItemState {
    pub fn new(json_data: &Value) -> Self {
        let mut all_null = true;

        let text = match json_data.get("text") {
            Some(v) => {
                all_null = false;
                v.to_string()
            }
            None => String::from(DEFAULT_TEXT),
        };
        let text_size = match json_data.get("textSize") {
            Some(v) => {
                all_null = false;
                v.as_i64().unwrap()
            }
            None => DEFAULT_TEXT_SIZE,
        };
        let text_color = match json_data.get("textColor") {
            Some(v) => {
                all_null = false;
                Self::extract_color(v)
            }
            None => DEFAULT_TEXT_COLOR,
        };
        let text_position = match json_data.get("textPosition") {
            Some(v) => {
                all_null = false;
                v.to_string()
            }
            None => String::from(DEFAULT_TEXT_POSITION),
        };
        let bg_color = match json_data.get("bgColor") {
            Some(v) => {
                all_null = false;
                Self::extract_color(v)
            }
            None => DEFAULT_BG_COLOR,
        };
        let icon_data = String::from("");
        // Uint8List? iconData;
        // if (json_data["icon"] != null) {
        //     let iconBase64 = json_data["icon"].toString();
        //     iconData = base64Decode(iconBase64.split(",").last);
        //     allNull = false;
        // }

        if all_null {
            return ItemState::empty();
        }

        ItemState {
            id: json_data["id"].as_i64().expect("ID must be numeric"),
            text,
            text_size,
            text_color,
            text_position,
            bg_color,
            icon_data,
        }
    }

    fn empty() -> Self {
        ItemState {
            id: -1,
            text: String::from(DEFAULT_TEXT),
            text_size: DEFAULT_TEXT_SIZE,
            text_color: DEFAULT_TEXT_COLOR,
            text_position: String::from(DEFAULT_TEXT_POSITION),
            bg_color: DEFAULT_BG_COLOR,
            icon_data: String::from(""),
        }
    }

    fn extract_color(color_string: &Value) -> HexColor {
        match color_string.as_str() {
            Some(s) => {
                let mut ss = String::from("#");
                ss.push_str(&s[2..8]);
                match HexColor::parse(&ss) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Error parsing {}: {}", s, e);
                        DEFAULT_TEXT_COLOR
                    }
                }
            }
            None => DEFAULT_TEXT_COLOR,
        }
    }
}
