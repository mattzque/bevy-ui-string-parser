use bevy::{render::color::Color, utils::HashMap};
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::multispace0 as multispace,
    character::streaming::char,
    combinator::{map, map_res},
    error::ParseError,
    number::complete::float,
    sequence::{delimited, preceded, tuple, Tuple},
    IResult, Parser,
};

lazy_static! {
    /// Table with Named Colors in CSS
    ///
    /// https://drafts.csswg.org/css-color/#named-colors
    pub static ref CSS_COLOR_TABLE: HashMap<&'static str, Color> = {
        const ERROR: &str = "Invalid named color entry!";
        HashMap::from([
            ("aliceblue", Color::hex("F0F8FF").expect(ERROR)),
            ("antiquewhite", Color::hex("FAEBD7").expect(ERROR)),
            ("aqua", Color::hex("00FFFF").expect(ERROR)),
            ("aquamarine", Color::hex("7FFFD4").expect(ERROR)),
            ("azure", Color::hex("F0FFFF").expect(ERROR)),
            ("beige", Color::hex("F5F5DC").expect(ERROR)),
            ("bisque", Color::hex("FFE4C4").expect(ERROR)),
            ("black", Color::hex("000000").expect(ERROR)),
            ("blanchedalmond", Color::hex("FFEBCD").expect(ERROR)),
            ("blue", Color::hex("0000FF").expect(ERROR)),
            ("blueviolet", Color::hex("8A2BE2").expect(ERROR)),
            ("brown", Color::hex("A52A2A").expect(ERROR)),
            ("burlywood", Color::hex("DEB887").expect(ERROR)),
            ("cadetblue", Color::hex("5F9EA0").expect(ERROR)),
            ("chartreuse", Color::hex("7FFF00").expect(ERROR)),
            ("chocolate", Color::hex("D2691E").expect(ERROR)),
            ("coral", Color::hex("FF7F50").expect(ERROR)),
            ("cornflowerblue", Color::hex("6495ED").expect(ERROR)),
            ("cornsilk", Color::hex("FFF8DC").expect(ERROR)),
            ("crimson", Color::hex("DC143C").expect(ERROR)),
            ("cyan", Color::hex("00FFFF").expect(ERROR)),
            ("darkblue", Color::hex("00008B").expect(ERROR)),
            ("darkcyan", Color::hex("008B8B").expect(ERROR)),
            ("darkgoldenrod", Color::hex("B8860B").expect(ERROR)),
            ("darkgray", Color::hex("A9A9A9").expect(ERROR)),
            ("darkgreen", Color::hex("006400").expect(ERROR)),
            ("darkgrey", Color::hex("A9A9A9").expect(ERROR)),
            ("darkkhaki", Color::hex("BDB76B").expect(ERROR)),
            ("darkmagenta", Color::hex("8B008B").expect(ERROR)),
            ("darkolivegreen", Color::hex("556B2F").expect(ERROR)),
            ("darkorange", Color::hex("FF8C00").expect(ERROR)),
            ("darkorchid", Color::hex("9932CC").expect(ERROR)),
            ("darkred", Color::hex("8B0000").expect(ERROR)),
            ("darksalmon", Color::hex("E9967A").expect(ERROR)),
            ("darkseagreen", Color::hex("8FBC8F").expect(ERROR)),
            ("darkslateblue", Color::hex("483D8B").expect(ERROR)),
            ("darkslategray", Color::hex("2F4F4F").expect(ERROR)),
            ("darkslategrey", Color::hex("2F4F4F").expect(ERROR)),
            ("darkturquoise", Color::hex("00CED1").expect(ERROR)),
            ("darkviolet", Color::hex("9400D3").expect(ERROR)),
            ("deeppink", Color::hex("FF1493").expect(ERROR)),
            ("deepskyblue", Color::hex("00BFFF").expect(ERROR)),
            ("dimgray", Color::hex("696969").expect(ERROR)),
            ("dimgrey", Color::hex("696969").expect(ERROR)),
            ("dodgerblue", Color::hex("1E90FF").expect(ERROR)),
            ("firebrick", Color::hex("B22222").expect(ERROR)),
            ("floralwhite", Color::hex("FFFAF0").expect(ERROR)),
            ("forestgreen", Color::hex("228B22").expect(ERROR)),
            ("fuchsia", Color::hex("FF00FF").expect(ERROR)),
            ("gainsboro", Color::hex("DCDCDC").expect(ERROR)),
            ("ghostwhite", Color::hex("F8F8FF").expect(ERROR)),
            ("gold", Color::hex("FFD700").expect(ERROR)),
            ("goldenrod", Color::hex("DAA520").expect(ERROR)),
            ("gray", Color::hex("808080").expect(ERROR)),
            ("green", Color::hex("008000").expect(ERROR)),
            ("greenyellow", Color::hex("ADFF2F").expect(ERROR)),
            ("grey", Color::hex("808080").expect(ERROR)),
            ("honeydew", Color::hex("F0FFF0").expect(ERROR)),
            ("hotpink", Color::hex("FF69B4").expect(ERROR)),
            ("indianred", Color::hex("CD5C5C").expect(ERROR)),
            ("indigo", Color::hex("4B0082").expect(ERROR)),
            ("ivory", Color::hex("FFFFF0").expect(ERROR)),
            ("khaki", Color::hex("F0E68C").expect(ERROR)),
            ("lavender", Color::hex("E6E6FA").expect(ERROR)),
            ("lavenderblush", Color::hex("FFF0F5").expect(ERROR)),
            ("lawngreen", Color::hex("7CFC00").expect(ERROR)),
            ("lemonchiffon", Color::hex("FFFACD").expect(ERROR)),
            ("lightblue", Color::hex("ADD8E6").expect(ERROR)),
            ("lightcoral", Color::hex("F08080").expect(ERROR)),
            ("lightcyan", Color::hex("E0FFFF").expect(ERROR)),
            ("lightgoldenrodyellow", Color::hex("FAFAD2").expect(ERROR)),
            ("lightgray", Color::hex("D3D3D3").expect(ERROR)),
            ("lightgreen", Color::hex("90EE90").expect(ERROR)),
            ("lightgrey", Color::hex("D3D3D3").expect(ERROR)),
            ("lightpink", Color::hex("FFB6C1").expect(ERROR)),
            ("lightsalmon", Color::hex("FFA07A").expect(ERROR)),
            ("lightseagreen", Color::hex("20B2AA").expect(ERROR)),
            ("lightskyblue", Color::hex("87CEFA").expect(ERROR)),
            ("lightslategray", Color::hex("778899").expect(ERROR)),
            ("lightslategrey", Color::hex("778899").expect(ERROR)),
            ("lightsteelblue", Color::hex("B0C4DE").expect(ERROR)),
            ("lightyellow", Color::hex("FFFFE0").expect(ERROR)),
            ("lime", Color::hex("00FF00").expect(ERROR)),
            ("limegreen", Color::hex("32CD32").expect(ERROR)),
            ("linen", Color::hex("FAF0E6").expect(ERROR)),
            ("magenta", Color::hex("FF00FF").expect(ERROR)),
            ("maroon", Color::hex("800000").expect(ERROR)),
            ("mediumaquamarine", Color::hex("66CDAA").expect(ERROR)),
            ("mediumblue", Color::hex("0000CD").expect(ERROR)),
            ("mediumorchid", Color::hex("BA55D3").expect(ERROR)),
            ("mediumpurple", Color::hex("9370DB").expect(ERROR)),
            ("mediumseagreen", Color::hex("3CB371").expect(ERROR)),
            ("mediumslateblue", Color::hex("7B68EE").expect(ERROR)),
            ("mediumspringgreen", Color::hex("00FA9A").expect(ERROR)),
            ("mediumturquoise", Color::hex("48D1CC").expect(ERROR)),
            ("mediumvioletred", Color::hex("C71585").expect(ERROR)),
            ("midnightblue", Color::hex("191970").expect(ERROR)),
            ("mintcream", Color::hex("F5FFFA").expect(ERROR)),
            ("mistyrose", Color::hex("FFE4E1").expect(ERROR)),
            ("moccasin", Color::hex("FFE4B5").expect(ERROR)),
            ("navajowhite", Color::hex("FFDEAD").expect(ERROR)),
            ("navy", Color::hex("000080").expect(ERROR)),
            ("oldlace", Color::hex("FDF5E6").expect(ERROR)),
            ("olive", Color::hex("808000").expect(ERROR)),
            ("olivedrab", Color::hex("6B8E23").expect(ERROR)),
            ("orange", Color::hex("FFA500").expect(ERROR)),
            ("orangered", Color::hex("FF4500").expect(ERROR)),
            ("orchid", Color::hex("DA70D6").expect(ERROR)),
            ("palegoldenrod", Color::hex("EEE8AA").expect(ERROR)),
            ("palegreen", Color::hex("98FB98").expect(ERROR)),
            ("paleturquoise", Color::hex("AFEEEE").expect(ERROR)),
            ("palevioletred", Color::hex("DB7093").expect(ERROR)),
            ("papayawhip", Color::hex("FFEFD5").expect(ERROR)),
            ("peachpuff", Color::hex("FFDAB9").expect(ERROR)),
            ("peru", Color::hex("CD853F").expect(ERROR)),
            ("pink", Color::hex("FFC0CB").expect(ERROR)),
            ("plum", Color::hex("DDA0DD").expect(ERROR)),
            ("powderblue", Color::hex("B0E0E6").expect(ERROR)),
            ("purple", Color::hex("800080").expect(ERROR)),
            ("rebeccapurple", Color::hex("663399").expect(ERROR)),
            ("red", Color::hex("FF0000").expect(ERROR)),
            ("rosybrown", Color::hex("BC8F8F").expect(ERROR)),
            ("royalblue", Color::hex("4169E1").expect(ERROR)),
            ("saddlebrown", Color::hex("8B4513").expect(ERROR)),
            ("salmon", Color::hex("FA8072").expect(ERROR)),
            ("sandybrown", Color::hex("F4A460").expect(ERROR)),
            ("seagreen", Color::hex("2E8B57").expect(ERROR)),
            ("seashell", Color::hex("FFF5EE").expect(ERROR)),
            ("sienna", Color::hex("A0522D").expect(ERROR)),
            ("silver", Color::hex("C0C0C0").expect(ERROR)),
            ("skyblue", Color::hex("87CEEB").expect(ERROR)),
            ("slateblue", Color::hex("6A5ACD").expect(ERROR)),
            ("slategray", Color::hex("708090").expect(ERROR)),
            ("slategrey", Color::hex("708090").expect(ERROR)),
            ("snow", Color::hex("FFFAFA").expect(ERROR)),
            ("springgreen", Color::hex("00FF7F").expect(ERROR)),
            ("steelblue", Color::hex("4682B4").expect(ERROR)),
            ("tan", Color::hex("D2B48C").expect(ERROR)),
            ("teal", Color::hex("008080").expect(ERROR)),
            ("thistle", Color::hex("D8BFD8").expect(ERROR)),
            ("tomato", Color::hex("FF6347").expect(ERROR)),
            ("turquoise", Color::hex("40E0D0").expect(ERROR)),
            ("violet", Color::hex("EE82EE").expect(ERROR)),
            ("wheat", Color::hex("F5DEB3").expect(ERROR)),
            ("white", Color::hex("FFFFFF").expect(ERROR)),
            ("whitesmoke", Color::hex("F5F5F5").expect(ERROR)),
            ("yellow", Color::hex("FFFF00").expect(ERROR)),
            ("yellowgreen", Color::hex("9ACD32").expect(ERROR)),
        ])
    };
}

/// Parses three floats, "1.0, 1.0, 1.0" into tuple of floats
fn three_float_parser(i: &str) -> IResult<&str, (f32, f32, f32)> {
    tuple((
        preceded(multispace, float),
        preceded(tuple((multispace, char(','), multispace)), float),
        preceded(tuple((multispace, char(','), multispace)), float),
    ))
    .parse(i)
}

/// Parses four floats, "1.0, 1.0, 1.0, 1.0" into tuple of floats
fn four_float_parser(i: &str) -> IResult<&str, (f32, f32, f32, f32)> {
    tuple((
        preceded(multispace, float),
        preceded(tuple((multispace, char(','), multispace)), float),
        preceded(tuple((multispace, char(','), multispace)), float),
        preceded(tuple((multispace, char(','), multispace)), float),
    ))
    .parse(i)
}

/// Generic parser for color functions, like rgb(1.0, 1.0, 1.0), etc.
///
/// Args:
/// name: the function name, for example "rgb" or "hsl"
fn color_fn_parser<'a, O, E>(
    name: &'static str,
    inner_parser: impl nom::Parser<&'a str, O, E>,
) -> impl FnMut(&'a str) -> Result<(&'a str, O), nom::Err<E>>
where
    E: ParseError<&'a str>,
{
    delimited(
        multispace,
        delimited(
            tuple((tag(name), tag("("), multispace)),
            inner_parser,
            tuple((multispace, tag(")"))),
        ),
        multispace,
    )
}

/// Parses rgb color function strings, like rgb(1.0, 1.0, 1.0)
fn color_rgb_parser(i: &str) -> IResult<&str, Color> {
    map(color_fn_parser("rgb", three_float_parser), |(r, g, b)| {
        Color::rgb(r, g, b)
    })
    .parse(i)
}

/// Parses rgba color function strings, like rgba(1.0, 1.0, 1.0, 1.0)
fn color_rgba_parser(i: &str) -> IResult<&str, Color> {
    map(color_fn_parser("rgba", four_float_parser), |(r, g, b, a)| {
        Color::rgba(r, g, b, a)
    })
    .parse(i)
}

/// Parses hsl color function strings, like hsl(1.0, 1.0, 1.0)
fn color_hsl_parser(i: &str) -> IResult<&str, Color> {
    map(color_fn_parser("hsl", three_float_parser), |(r, g, b)| {
        Color::hsl(r, g, b)
    })
    .parse(i)
}

/// Parses hsla color function strings, like hsla(1.0, 1.0, 1.0, 1.0)
fn color_hsla_parser(i: &str) -> IResult<&str, Color> {
    map(color_fn_parser("hsla", four_float_parser), |(r, g, b, a)| {
        Color::hsla(r, g, b, a)
    })
    .parse(i)
}

/// Parses a byte hex string, like "FF"
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

/// Parses a hex string character interpreted as a byte, like "F" -> "FF" -> 255
fn from_half_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(format!("{}{}", input, input).as_str(), 16)
}

/// Returns true if the character is a valid hexadecimal character
fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

/// Takes a two letter hexadecimal from the input and return it as a byte
fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

/// Takes a single letter hexadecimal from the input and return it as a byte
fn hex_half(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 1, is_hex_digit), from_half_hex).parse(input)
}

/// Takes a 6 character hexadecimal color prefixed with `#` and parses it to a Color
///
/// For example: `#FF0000`
fn color_hex6_parser(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color::rgb_u8(r, g, b)))
}

/// Takes a 8 character hexadecimal color prefixed with `#` and parses it to a Color
///
/// For example: `#FF0000FF`
fn color_hex8_parser(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b, a)) = (hex_primary, hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color::rgba_u8(r, g, b, a)))
}

/// Takes a 3 character hexadecimal color prefixed with `#` and parses it to a Color
///
/// For example: `#F00`
fn color_hex3_parser(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = (hex_half, hex_half, hex_half).parse(input)?;
    Ok((input, Color::rgb_u8(r, g, b)))
}

/// Takes a string found in the css color table and return its color
fn color_css_names_parser(input: &str) -> IResult<&str, Color> {
    if let Some(color) = CSS_COLOR_TABLE.get(input.trim()) {
        Ok(("", *color))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

/// Parser for strings that represent a [`bevy::render::color::Color`].
///
/// The syntax is inspired by CSS:
///
/// * `red, blue -> css color names (see https://drafts.csswg.org/css-color/#named-colors)
/// * `#f0f`, `#ff00ff` -> hex color (3 or 6 digits)
/// * `#ff00ff00` -> hex color with alpha (8 digits)
/// * `rgb(1.0, 0.0, 0.0)` -> rgb color (0.0-1.0)
/// * `rgba(1.0, 0.0, 0.0, 1.0)` -> rgb color with alpha (0.0-1.0)
/// * `hsl(0.0, 1.0, 0.5)` -> hsl color (0.0-1.0)
/// * `hsla(0.0, 1.0, 0.5, 1.0)` -> hsl color with alpha (0.0-1.0)
///
pub fn color_parser(input: &str) -> IResult<&str, Color> {
    delimited(
        multispace,
        alt((
            color_rgb_parser,
            color_rgba_parser,
            color_hsl_parser,
            color_hsla_parser,
            color_hex8_parser,
            color_hex6_parser,
            color_hex3_parser,
            color_css_names_parser,
        )),
        multispace,
    )(input)
}

/// Wrapper for [`color_parser`] that returns an optional [`bevy::render::color::Color`]
pub fn color_string_parser(input: &str) -> Option<Color> {
    color_parser(input).map(|(_, value)| value).ok()
}

/// Wrapper for [`angle_parser`] that implements a serde deserializer
#[cfg(feature = "serde")]
pub fn color_serde_parser<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    color_string_parser(s).ok_or(D::Error::custom("invalid color string"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("#FF0000", Color::RED ; "hex6 red")]
    #[test_case("#FF0000FF", Color::RED ; "hex8 red")]
    #[test_case("#F00", Color::RED ; "hex3 red")]
    #[test_case("#f00", Color::RED ; "hex3 red lowercase")]
    #[test_case("rgb(1.0, 0, 0)", Color::RED ; "rgb red")]
    #[test_case("rgba(1.0, 0, 0, 1)", Color::RED ; "rgba red")]
    #[test_case("hsl(0, 1.0, 0.5)", Color::RED.as_hsla() ; "hsl red")]
    #[test_case("hsla(0, 1.0, 0.5, 1)", Color::RED.as_hsla() ; "hsla red")]
    #[test_case("red", Color::RED ; "css name red")]
    #[test_case("fuchsia", Color::FUCHSIA ; "css name fuchsia")]
    fn test_color_parser_variants(string: &str, expected: Color) {
        assert_eq!(color_parser(string), Ok(("", expected)));
    }

    #[test]
    fn test_color_parser() {
        assert_eq!(color_parser("  red"), Ok(("", Color::RED)));
        assert_eq!(color_parser("red  "), Ok(("", Color::RED)));
        assert_eq!(color_parser(" red "), Ok(("", Color::RED)));
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests_serde {
    use bevy::render::color::Color;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(deserialize_with = "super::color_serde_parser")]
        pub color: Color,
    }

    #[test]
    fn test_angle_serde_parser() {
        let foo: Foo = serde_json::from_str(r#"{"color": "red"}"#).unwrap();
        assert_eq!(foo.color, Color::RED);
    }
}