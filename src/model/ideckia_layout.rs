use crate::model::item_state::ItemState;
use serde_json::Value;

pub struct IdeckiaLayout {
    pub rows: i64,
    pub columns: i64,
    pub items: Vec<ItemState>,
    pub fixed_items: Vec<ItemState>,
}

impl IdeckiaLayout {
    pub fn new(json_data: &Value) -> Self {
        let mut items: Vec<ItemState> = Vec::new();
        let layout_items = json_data["items"].as_array().unwrap();
        for i in layout_items.iter() {
            items.push(ItemState::new(i));
        }

        let mut fixed_items: Vec<ItemState> = Vec::new();
        match json_data["fixedItems"].as_array() {
            Some(layout_fixed_items) => {
                for i in layout_fixed_items.iter() {
                    fixed_items.push(ItemState::new(i));
                }
            }
            None => {}
        };

        IdeckiaLayout {
            rows: json_data["rows"].as_i64().unwrap(),
            columns: json_data["columns"].as_i64().unwrap(),
            items: items,
            fixed_items: fixed_items,
        }
    }
}
