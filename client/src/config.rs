/// Base URL for API calls. Empty string means same-origin (relative URLs).
/// Set via `API_BASE_URL` env var at compile time for cross-origin deployments.
pub fn api_base_url() -> &'static str {
    option_env!("API_BASE_URL").unwrap_or("")
}

#[derive(Clone, Copy, PartialEq)]
pub enum RankStyle {
    European,
    American,
}

/// Map an internal rank to its display rank based on the ranking style.
///
/// EU: 1=Marshal(highest) .. 9=Scout(lowest), S=Spy
/// US: 10=Marshal(highest) .. 2=Scout(lowest), 1=Spy
pub fn display_rank(internal: &str, style: RankStyle) -> &'static str {
    if style == RankStyle::European {
        match internal {
            "1" => "1",
            "2" => "2",
            "3" => "3",
            "4" => "4",
            "5" => "5",
            "6" => "6",
            "7" => "7",
            "8" => "8",
            "9" => "9",
            "S" => "S",
            "B" => "B",
            "F" => "F",
            _ => "U",
        }
    } else {
        match internal {
            "1" => "10",
            "2" => "9",
            "3" => "8",
            "4" => "7",
            "5" => "6",
            "6" => "5",
            "7" => "4",
            "8" => "3",
            "9" => "2",
            "S" => "1",
            "B" => "B",
            "F" => "F",
            _ => "U",
        }
    }
}

pub fn load_rank_style() -> RankStyle {
    let win = web_sys::window().unwrap();
    if let Ok(Some(storage)) = win.local_storage() {
        if let Ok(Some(val)) = storage.get_item("rankStyle") {
            if val == "american" {
                return RankStyle::American;
            }
        }
    }
    RankStyle::European
}

pub fn save_rank_style(style: RankStyle) {
    let win = web_sys::window().unwrap();
    if let Ok(Some(storage)) = win.local_storage() {
        let val = match style {
            RankStyle::European => "european",
            RankStyle::American => "american",
        };
        let _ = storage.set_item("rankStyle", val);
    }
}
