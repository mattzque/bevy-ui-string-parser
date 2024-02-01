# Bevy-UI String Parser Utilities

![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy_ui_string_parser)
[![Docs](https://docs.rs/bevy/badge.svg)](https://docs.rs/bevy_ui_string_parser/latest/bevy_ui_string_parser/)

This rust crate provides a set of parsing functions that can be used to
convert strings into various bevy-ui types like `Color`, `Val` or `UiRect`.
The syntax matches CSS, but it doesn't try to replicate it perfectly.

It depends on the [nom](https://crates.io/crates/nom) library for parsing.

## Parsers

### Color

Parses Color values, such as `#f0f` or `yellow`.

```rust
use bevy::render::color::Color;
use bevy_ui_string_parser::color_string_parser;
let color: Option<Color> = color_string_parser("#f00");
assert_eq!(color, Some(Color::RED));
```

#### Supported syntax

* `red`, `blue` -> css color names (see https://drafts.csswg.org/css-color/#named-colors)
* `#f0f`, `#ff00ff` -> hex color (3 or 6 digits)
* `#ff00ff00` -> hex color with alpha (8 digits)
* `rgb(1.0, 0.0, 0.0)` -> rgb color (0.0-1.0)
* `rgba(1.0, 0.0, 0.0, 1.0)` -> rgb color with alpha (0.0-1.0)
* `hsl(0.0, 1.0, 0.5)` -> hsl color (0.0-1.0)
* `hsla(0.0, 1.0, 0.5, 1.0)` -> hsl color with alpha (0.0-1.0)

### Val

Parses Val values, such as `1px` or `50%`.

```rust
use bevy::ui::Val;
use bevy_ui_string_parser::val_string_parser;
let value: Option<Val> = val_string_parser("45%");
assert_eq!(value, Some(Val::Percent(45.0)));
```

#### Supported syntax

* `auto` -> `Val::Auto`
* `12px` -> `Val::Px(12.0)`
* `12%` -> `Val::Percent(12.0)`
* `12vw` -> `Val::Vw(12.0)`
* `12vh` -> `Val::Vh(12.0)`
* `12vmin` -> `Val::VMin(12.0)`
* `12vmax` -> `Val::VMax(12.0)`

### Rect

Parses UiRect values, such as `25px 50px`.

This uses the same ordering as CSS properties like `padding` or `margin`,
see [mdn](https://developer.mozilla.org/en-US/docs/Web/CSS/padding) for more information.

```rust
use bevy::ui::{UiRect, Val};
use bevy_ui_string_parser::rect_string_parser;
let rect: Option<UiRect> = rect_string_parser("50px 100px");
assert_eq!(rect, Some(UiRect::new(
    Val::Px(100.0), Val::Px(100.0), Val::Px(50.0), Val::Px(50.0))));
```

#### Supported syntax

* `10px 20px 30px 40px` -> `top | right | bottom | left`
* `10px 20px 10px` -> `top | left and right | bottom`
* `10px 20px` -> `top and bottom | left and right`
* `10px` -> `top, right, bottom and left`

### Angle

Parses angles into float values, such as `180deg`, returns radians.

```rust
use bevy_ui_string_parser::angle_string_parser;
let angle: Option<f32> = angle_string_parser("180deg");
assert_eq!(angle, Some(180.0_f32.to_radians()));
```

#### Supported syntax

* `180deg` -> `3.14159265359`
* `1.3rad` -> `1.3`
* `1.3` -> `1.3`

## Serde

Each parser also provides a serde deserializer `*_serde_parser`, for example:

```rust
use serde::Deserialize;
use std::f32::consts::PI;

#[derive(Deserialize)]
pub struct Foo {
    #[serde(deserialize_with = "bevy_ui_string_parser::angle_serde_parser")]
    pub angle: f32,
}

let foo: Foo = serde_json::from_str(r#"{"angle": "180deg"}"#).unwrap();
assert_eq!(foo.angle, PI);
```

It requires the optional `serde` feature.

## Changelog

* `v0.1.2` made serde feature optional
* `v0.1.1` readme added
* `v0.1.0` initial release