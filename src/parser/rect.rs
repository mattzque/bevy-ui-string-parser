use bevy::ui;
use nom::{
    branch::alt,
    character::complete::multispace0 as multispace,
    combinator::{complete, map},
    sequence::{preceded, tuple},
    IResult,
};

use super::val_parser;

/// Parse rect from a string of four val strings
///
/// Corresponds to the same order used in CSS padding/margin/etc.
///
/// top | right | bottom | left
fn four_rect_parser(input: &str) -> IResult<&str, ui::UiRect> {
    complete(map(
        tuple((
            preceded(multispace, val_parser),
            preceded(multispace, val_parser),
            preceded(multispace, val_parser),
            preceded(multispace, val_parser),
        )),
        |(top, right, bottom, left)| ui::UiRect::new(left, right, top, bottom),
    ))(input)
}

/// Parse rect from a string of three val strings
///
/// Corresponds to the same order used in CSS padding/margin/etc.
///
/// top | left and right | bottom
fn three_rect_parser(input: &str) -> IResult<&str, ui::UiRect> {
    complete(map(
        tuple((
            preceded(multispace, val_parser),
            preceded(multispace, val_parser),
            preceded(multispace, val_parser),
        )),
        |(top, left_right, bottom)| ui::UiRect::new(left_right, left_right, top, bottom),
    ))(input)
}

/// Parse rect from a string of two val strings
///
/// Corresponds to the same order used in CSS padding/margin/etc.
///
/// top and bottom | left and right
fn two_rect_parser(input: &str) -> IResult<&str, ui::UiRect> {
    complete(map(
        tuple((
            preceded(multispace, val_parser),
            preceded(multispace, val_parser),
        )),
        |(top_bottom, left_right)| ui::UiRect::new(left_right, left_right, top_bottom, top_bottom),
    ))(input)
}

/// Parse rect from a string of a single val string
///
/// Corresponds to the same order used in CSS padding/margin/etc.
///
/// top, right, bottom and left
fn one_rect_parser(input: &str) -> IResult<&str, ui::UiRect> {
    complete(map(preceded(multispace, val_parser), ui::UiRect::all))(input)
}

/// Parse [`bevy::ui::UiRect`] from a string of a single val strings
///
/// Corresponds to the same order used in CSS padding/margin/etc.
///
/// Either one, two, three or four val strings can be given:
/// * top | right | bottom | left
/// * top | left and right | bottom
/// * top and bottom | left and right
/// * top, right, bottom and left
pub fn rect_parser(input: &str) -> IResult<&str, ui::UiRect> {
    alt((four_rect_parser, three_rect_parser, two_rect_parser, one_rect_parser))(input)
}

/// Wrapper for [`rect_parser`] that returns an optional [`bevy::ui::Val`]
pub fn rect_string_parser(input: &str) -> Option<ui::UiRect> {
    rect_parser(input).map(|(_, value)| value).ok()
}

/// Wrapper for [`rect_parser`] that implements a serde deserializer
#[cfg(feature = "serde")]
pub fn rect_serde_parser<'de, D>(deserializer: D) -> Result<ui::UiRect, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    rect_string_parser(s).ok_or(D::Error::custom("invalid rect string"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_parser() {
        assert_eq!(
            rect_parser("auto auto auto auto"),
            Ok(("", ui::UiRect::all(ui::Val::Auto)))
        );
        assert_eq!(rect_parser("auto auto auto"), Ok(("", ui::UiRect::all(ui::Val::Auto))));
        assert_eq!(rect_parser("auto auto"), Ok(("", ui::UiRect::all(ui::Val::Auto))));
        assert_eq!(rect_parser("auto"), Ok(("", ui::UiRect::all(ui::Val::Auto))));
        assert_eq!(
            rect_parser("1px 2px 3px 4px"),
            Ok((
                "",
                ui::UiRect::new(ui::Val::Px(4.0), ui::Val::Px(2.0), ui::Val::Px(1.0), ui::Val::Px(3.0))
            ))
        );
        assert_eq!(
            rect_parser("1px 2px 3px"),
            Ok((
                "",
                ui::UiRect::new(ui::Val::Px(2.0), ui::Val::Px(2.0), ui::Val::Px(1.0), ui::Val::Px(3.0))
            ))
        );
        assert_eq!(
            rect_parser("1px 2px"),
            Ok((
                "",
                ui::UiRect::new(ui::Val::Px(2.0), ui::Val::Px(2.0), ui::Val::Px(1.0), ui::Val::Px(1.0))
            ))
        );
        assert_eq!(
            rect_parser("1px"),
            Ok((
                "",
                ui::UiRect::all(ui::Val::Px(1.0))
            ))
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests_serde {
    use bevy::ui::{UiRect, Val};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(deserialize_with = "super::rect_serde_parser")]
        pub rect: UiRect,
    }

    #[test]
    fn test_angle_serde_parser() {
        let foo: Foo = serde_json::from_str(r#"{"rect": "42px"}"#).unwrap();
        assert_eq!(foo.rect, UiRect::all(Val::Px(42.0)));
    }
}