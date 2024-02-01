use nom::{
    branch::alt, bytes::complete::tag, combinator::map, number::complete::float, sequence::tuple,
    IResult,
};

/// Parser for a angle value string.
///
/// Supported Formats:
/// * 60deg / 10.234deg / -45deg (interpreted as degrees, converted to radians)
/// * 3.1415rad (is interpreted as radians)
/// * 3.1415 (is interpreted as radians)
///
/// TODO: support grad, turn suffixes, f64 version?
/// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
pub fn angle_parser(input: &str) -> IResult<&str, f32> {
    alt((
        map(tuple((float, tag("deg"))), |(val, _)| val.to_radians()),
        map(tuple((float, tag("rad"))), |(val, _)| val),
        map(float, |val| val),
    ))(input.trim())
}

/// Wrapper for [`angle_parser`] that returns an optional f32
pub fn angle_string_parser(input: &str) -> Option<f32> {
    angle_parser(input).map(|(_, value)| value).ok()
}

/// Wrapper for [`angle_parser`] that implements a serde deserializer
#[cfg(feature = "serde")]
pub fn angle_serde_parser<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    angle_string_parser(s).ok_or(D::Error::custom("invalid angle string"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("180", 180.0 ; "with full integer string")]
    #[test_case(".1", 0.1 ; "with prefix period string")]
    #[test_case("1.", 1.0 ; "with suffix period string")]
    #[test_case("1.1", 1.1 ; "with full float string")]
    #[test_case("42.42", 42.42 ; "with two digit full float string")]
    #[test_case("0", 0. ; "with zero string")]
    #[test_case("0.0", 0. ; "with full zero string")]
    #[test_case(".0", 0. ; "with prefix zero string")]
    #[test_case("0.", 0. ; "with suffix zero string")]
    fn test_angle_string_parser(string: &str, expected: f32) {
        assert_eq!(
            angle_parser(&format!("{}deg", string)),
            Ok(("", expected.to_radians()))
        );
        assert_eq!(angle_parser(&format!("{}rad", string)), Ok(("", expected)));
        assert_eq!(angle_parser(string), Ok(("", expected)));
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests_serde {
    use serde::Deserialize;
    use std::f32::consts::PI;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(deserialize_with = "super::angle_serde_parser")]
        pub angle: f32,
    }

    #[test]
    fn test_angle_serde_parser() {
        let foo: Foo = serde_json::from_str(r#"{"angle": "180deg"}"#).unwrap();
        assert_eq!(foo.angle, PI);
    }
}
