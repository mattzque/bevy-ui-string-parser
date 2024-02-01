use bevy::ui::Val;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0 as multispace,
    combinator::map,
    number::streaming::float,
    sequence::{delimited, tuple},
    IResult,
};

/// Parser for strings that represent a [`bevy::ui::Val`].
///
/// The syntax is inspired by CSS:
///
/// * `auto` -> Val::Auto
/// * `12px` -> Val::Px(12.0)
/// * `12%` -> Val::Percent(12.0)
/// * `12vw` -> Val::Vw(12.0)
/// * `12vh` -> Val::Vh(12.0)
/// * `12vmin` -> Val::VMin(12.0)
/// * `12vmax` -> Val::VMax(12.0)
///
pub fn val_parser(input: &str) -> IResult<&str, Val> {
    delimited(
        multispace,
        alt((
            map(tag("auto"), |_| Val::Auto),
            map(tuple((float, tag("px"))), |(val, _)| Val::Px(val)),
            map(tuple((float, tag("%"))), |(val, _)| Val::Percent(val)),
            map(tuple((float, tag("vw"))), |(val, _)| Val::Vw(val)),
            map(tuple((float, tag("vh"))), |(val, _)| Val::Vh(val)),
            map(tuple((float, tag("vmin"))), |(val, _)| Val::VMin(val)),
            map(tuple((float, tag("vmax"))), |(val, _)| Val::VMax(val)),
        )),
        multispace,
    )(input)
}

/// Wrapper for [`val_parser`] that returns an optional [`bevy::ui::Val`]
pub fn val_string_parser(input: &str) -> Option<Val> {
    val_parser(input).map(|(_, value)| value).ok()
}

/// Wrapper for [`val_parser`] that implements a serde deserializer
#[cfg(feature = "serde")]
pub fn val_serde_parser<'de, D>(deserializer: D) -> Result<Val, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    val_string_parser(s).ok_or(D::Error::custom("invalid val string"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1000", 1000. ; "with full integer string")]
    #[test_case(".1", 0.1 ; "with prefix period string")]
    #[test_case("1.", 1.0 ; "with suffix period string")]
    #[test_case("10.", 10.0 ; "with ten suffix period string")]
    #[test_case("1.1", 1.1 ; "with full float string")]
    #[test_case("42.42", 42.42 ; "with two digit full float string")]
    #[test_case("0", 0. ; "with zero string")]
    #[test_case("0.0", 0. ; "with full zero string")]
    #[test_case(".0", 0. ; "with prefix zero string")]
    #[test_case("0.", 0. ; "with suffix zero string")]
    fn test_val_string_parser_floats(string: &str, expected: f32) {
        assert_eq!(
            val_parser(&format!("{}px", string)),
            Ok(("", Val::Px(expected)))
        );
        assert_eq!(
            val_parser(&format!("{}%", string)),
            Ok(("", Val::Percent(expected)))
        );
        assert_eq!(
            val_parser(&format!("{}vw", string)),
            Ok(("", Val::Vw(expected)))
        );
        assert_eq!(
            val_parser(&format!("{}vh", string)),
            Ok(("", Val::Vh(expected)))
        );
        assert_eq!(
            val_parser(&format!("{}vmin", string)),
            Ok(("", Val::VMin(expected)))
        );
        assert_eq!(
            val_parser(&format!("{}vmax", string)),
            Ok(("", Val::VMax(expected)))
        );
    }

    #[test]
    fn test_val_string_parser() {
        assert_eq!(val_parser("  auto"), Ok(("", Val::Auto)));
        assert_eq!(val_parser("auto  "), Ok(("", Val::Auto)));
        assert_eq!(val_parser("  auto  "), Ok(("", Val::Auto)));
        assert_eq!(val_parser("auto"), Ok(("", Val::Auto)));
        assert_eq!(val_parser("1px"), Ok(("", Val::Px(1.0))));
        assert_eq!(val_parser("1.0px"), Ok(("", Val::Px(1.0))));
        assert_eq!(val_parser("1.0%"), Ok(("", Val::Percent(1.0))));
        assert_eq!(val_parser("1.0vw"), Ok(("", Val::Vw(1.0))));
        assert_eq!(val_parser("1.32vh"), Ok(("", Val::Vh(1.32))));
        assert_eq!(val_parser("1.0vmin"), Ok(("", Val::VMin(1.0))));
        assert_eq!(val_parser("1.32vmax"), Ok(("", Val::VMax(1.32))));
        assert!(val_parser("1.32").is_err());
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests_serde {
    use bevy::ui::Val;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(deserialize_with = "super::val_serde_parser")]
        pub value: Val,
    }

    #[test]
    fn test_angle_serde_parser() {
        let foo: Foo = serde_json::from_str(r#"{"value": "42px"}"#).unwrap();
        assert_eq!(foo.value, Val::Px(42.0));
    }
}
