mod angle;
mod color;
mod rect;
mod val;

#[cfg(feature = "serde")]
pub use angle::angle_serde_parser;
#[cfg(feature = "serde")]
pub use color::color_serde_parser;
#[cfg(feature = "serde")]
pub use rect::rect_serde_parser;
#[cfg(feature = "serde")]
pub use val::val_serde_parser;
pub use angle::{angle_parser, angle_string_parser};
pub use color::CSS_COLOR_TABLE;
pub use color::{color_parser, color_string_parser};
pub use rect::{rect_parser, rect_string_parser};
pub use val::{val_parser, val_string_parser};
