// Error handling
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const DEFAULT_TEXT_MODEL: &str = "gemma3:1b";

    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
        Only give user messages, not system messages.
    "#;
}

pub fn get_text_model() -> String {
    if let Some(suggested_model) = std::env::args().nth(1) {
        return suggested_model;
    }

    return format!("{}", consts::DEFAULT_TEXT_MODEL);
}