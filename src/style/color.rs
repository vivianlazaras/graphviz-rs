//! This module defines the `Color` enum used for representing colors in Graphviz-compatible ways.
//!
//! It supports parsing from RGB, hex, and YCbCr formats, as well as rendering
//! to normalized RGB or hex string formats for use in Graphviz outputs.

use std::fmt;
use std::str::FromStr;

/// Represents a color value, either in RGB or Hex string format.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use graphviz::style::color::Color;
/// use std::str::FromStr;
///
/// let red = Color::from_str("#ff0000").unwrap();
/// assert_eq!(red.to_hex_string(), "#ff0000");
///
/// let green = Color::from_str("rgb(0,255,0)").unwrap();
/// assert_eq!(green.to_hex_string(), "#00ff00");
///
/// let blue = Color::from_ycbcr(100.0, 100.0, 200.0);
/// println!("Blue as RGB: {}", blue);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum Color {
    /// Raw RGB triplet.
    RGB(u8, u8, u8),
    /// Hex string (e.g., "#ff0000")
    Hex(String),
}

impl Color {
    /// CSS color constants

    pub const ALICEBLUE: Color = Color::RGB(240, 248, 255);
    pub const ANTIQUEWHITE: Color = Color::RGB(250, 235, 215);
    pub const AQUA: Color = Color::RGB(0, 255, 255);
    pub const AQUAMARINE: Color = Color::RGB(127, 255, 212);
    pub const AZURE: Color = Color::RGB(240, 255, 255);
    pub const BEIGE: Color = Color::RGB(245, 245, 220);
    pub const BISQUE: Color = Color::RGB(255, 228, 196);
    pub const BLACK: Color = Color::RGB(0, 0, 0);
    pub const BLANCHEDALMOND: Color = Color::RGB(255, 235, 205);
    pub const BLUE: Color = Color::RGB(0, 0, 255);
    pub const BLUEVIOLET: Color = Color::RGB(138, 43, 226);
    pub const BROWN: Color = Color::RGB(165, 42, 42);
    pub const BURLYWOOD: Color = Color::RGB(222, 184, 135);
    pub const CADETBLUE: Color = Color::RGB(95, 158, 160);
    pub const CHARTREUSE: Color = Color::RGB(127, 255, 0);
    pub const CHOCOLATE: Color = Color::RGB(210, 105, 30);
    pub const CORAL: Color = Color::RGB(255, 127, 80);
    pub const CORNFLOWERBLUE: Color = Color::RGB(100, 149, 237);
    pub const CORNSILK: Color = Color::RGB(255, 248, 220);
    pub const CRIMSON: Color = Color::RGB(220, 20, 60);
    pub const CYAN: Color = Color::RGB(0, 255, 255);
    pub const DARKBLUE: Color = Color::RGB(0, 0, 139);
    pub const DARKCYAN: Color = Color::RGB(0, 139, 139);
    pub const DARKGOLDENROD: Color = Color::RGB(184, 134, 11);
    pub const DARKGRAY: Color = Color::RGB(169, 169, 169);
    pub const DARKGREEN: Color = Color::RGB(0, 100, 0);
    pub const DARKGREY: Color = Color::RGB(169, 169, 169);
    pub const DARKKHAKI: Color = Color::RGB(189, 183, 107);
    pub const DARKMAGENTA: Color = Color::RGB(139, 0, 139);
    pub const DARKOLIVEGREEN: Color = Color::RGB(85, 107, 47);
    pub const DARKORANGE: Color = Color::RGB(255, 140, 0);
    pub const DARKORCHID: Color = Color::RGB(153, 50, 204);
    pub const DARKRED: Color = Color::RGB(139, 0, 0);
    pub const DARKSALMON: Color = Color::RGB(233, 150, 122);
    pub const DARKSEAGREEN: Color = Color::RGB(143, 188, 143);
    pub const DARKSLATEBLUE: Color = Color::RGB(72, 61, 139);
    pub const DARKSLATEGRAY: Color = Color::RGB(47, 79, 79);
    pub const DARKSLATEGREY: Color = Color::RGB(47, 79, 79);
    pub const DARKTURQUOISE: Color = Color::RGB(0, 206, 209);
    pub const DARKVIOLET: Color = Color::RGB(148, 0, 211);
    pub const DEEPPINK: Color = Color::RGB(255, 20, 147);
    pub const DEEPSKYBLUE: Color = Color::RGB(0, 191, 255);
}

impl Color {
    /// Creates a `Color::RGB` from YCbCr values by converting to RGB.
    ///
    /// # Arguments
    ///
    /// * `y` - Luma (brightness)
    /// * `cb` - Blue-difference chroma
    /// * `cr` - Red-difference chroma
    ///
    /// # Returns
    ///
    /// * `Color::RGB(r, g, b)` where each channel is in range 0–255
    ///
    /// # Example
    ///
    /// ```
    /// use graphviz::style::color::Color;
    ///
    /// let color = Color::from_ycbcr(100.0, 100.0, 100.0);
    /// assert_eq!(color, Color::RGB(60, 129, 50));
    /// ```

    pub fn from_ycbcr(y: f32, cb: f32, cr: f32) -> Self {
        let r = (y + 1.402 * (cr - 128.0)).clamp(0.0, 255.0) as u8;
        let g = (y - 0.344136 * (cb - 128.0) - 0.714136 * (cr - 128.0)).clamp(0.0, 255.0) as u8;
        let b = (y + 1.772 * (cb - 128.0)).clamp(0.0, 255.0) as u8;
        Self::RGB(r, g, b)
    }

    /// Returns normalized RGB values in 0.0–1.0 range.
    ///
    /// If the variant is `Hex`, it is parsed internally to RGB.
    ///
    /// # Example
    ///
    /// ```
    /// use graphviz::style::color::Color;
    /// let color = Color::RGB(128, 64, 32);
    /// let (r, g, b) = color.as_rgb_normalized();
    /// assert!((r - 0.501).abs() < 0.01);
    /// ```
    pub fn as_rgb_normalized(&self) -> (f32, f32, f32) {
        match self {
            Color::RGB(r, g, b) => (*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0),
            Color::Hex(s) => {
                let rgb = Color::from_str(s).unwrap_or(Color::RGB(0, 0, 0));
                rgb.as_rgb_normalized()
            }
        }
    }

    /// Returns a `#rrggbb`-style hex string.
    ///
    /// # Example
    ///
    /// ```
    /// use graphviz::style::color::Color;
    /// let color = Color::RGB(255, 255, 0);
    /// assert_eq!(color.to_hex_string(), "#ffff00");
    /// ```
    pub fn to_hex_string(&self) -> String {
        match self {
            Color::RGB(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
            Color::Hex(s) => s.clone(),
        }
    }

    /// Returns the color as a quoted Graphviz-compatible string (e.g., `"#ff00ff"`).
    ///
    /// # Example
    ///
    /// ```
    /// use graphviz::style::color::Color;
    /// let color = Color::RGB(12, 34, 56);
    /// assert_eq!(color.to_graphviz(), "\"#0c2238\"");
    /// ```
    pub fn to_graphviz(&self) -> String {
        format!("\"{}\"", self.to_hex_string())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl FromStr for Color {
    type Err = &'static str;

    /// Parses a string into a `Color`.
    ///
    /// Supported formats:
    /// - `"#rrggbb"` or `"#rgb"` (hex color)
    /// - `"rgb(r, g, b)"` where `r`, `g`, `b` are integers 0–255
    /// - `"ycbcr(y, cb, cr)"` where values are floats (0–255)
    ///
    /// # Examples
    ///
    /// ```
    /// use graphviz::style::color::Color;
    /// use std::str::FromStr;
    ///
    /// let red = Color::from_str("#f00").unwrap();
    /// assert_eq!(red, Color::RGB(255, 0, 0));
    ///
    /// let green = Color::from_str("rgb(0, 255, 0)").unwrap();
    /// assert_eq!(green, Color::RGB(0, 255, 0));
    ///
    /// let converted = Color::from_str("ycbcr(100.0, 100.0, 100.0)").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with('#') {
            let hex = s.trim_start_matches('#');
            let hex = match hex.len() {
                3 => hex.chars().flat_map(|c| [c, c]).collect::<String>(),
                6 => hex.to_string(),
                _ => return Err("Invalid hex color"),
            };
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "bad hex")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "bad hex")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "bad hex")?;
            Ok(Color::RGB(r, g, b))
        } else if s.starts_with("rgb(") {
            let nums: Vec<&str> = s
                .trim_start_matches("rgb(")
                .trim_end_matches(')')
                .split(',')
                .collect();
            if nums.len() != 3 {
                return Err("Invalid rgb format");
            }
            let r = nums[0].trim().parse::<u8>().map_err(|_| "bad rgb")?;
            let g = nums[1].trim().parse::<u8>().map_err(|_| "bad rgb")?;
            let b = nums[2].trim().parse::<u8>().map_err(|_| "bad rgb")?;
            Ok(Color::RGB(r, g, b))
        } else if s.starts_with("ycbcr(") {
            let nums: Vec<&str> = s
                .trim_start_matches("ycbcr(")
                .trim_end_matches(')')
                .split(',')
                .collect();
            if nums.len() != 3 {
                return Err("Invalid ycbcr format");
            }
            let y = nums[0].trim().parse::<f32>().map_err(|_| "bad ycbcr")?;
            let cb = nums[1].trim().parse::<f32>().map_err(|_| "bad ycbcr")?;
            let cr = nums[2].trim().parse::<f32>().map_err(|_| "bad ycbcr")?;
            Ok(Color::from_ycbcr(y, cb, cr))
        } else {
            Err("Unknown color format")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_ycbcr() {
        let color = Color::from_ycbcr(100.0, 100.0, 100.0);
        assert_eq!(color, Color::RGB(60, 129, 50));
    }

    #[test]
    fn test_as_rgb_normalized_rgb() {
        let color = Color::RGB(128, 64, 32);
        let (r, g, b) = color.as_rgb_normalized();
        assert!((r - 0.501).abs() < 0.01);
        assert!((g - 0.251).abs() < 0.01);
        assert!((b - 0.125).abs() < 0.01);
    }

    #[test]
    fn test_as_rgb_normalized_hex() {
        let color = Color::Hex("#ff0000".to_string());
        let (r, g, b) = color.as_rgb_normalized();
        assert!((r - 1.0).abs() < 0.01);
        assert!((g - 0.0).abs() < 0.01);
        assert!((b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_to_hex_string_rgb() {
        let color = Color::RGB(255, 0, 255);
        assert_eq!(color.to_hex_string(), "#ff00ff");
    }

    #[test]
    fn test_to_hex_string_hex() {
        let color = Color::Hex("#abcdef".to_string());
        assert_eq!(color.to_hex_string(), "#abcdef");
    }

    #[test]
    fn test_to_graphviz() {
        let color = Color::RGB(12, 34, 56);
        assert_eq!(color.to_graphviz(), "\"#0c2238\"");
    }

    #[test]
    fn test_display_trait() {
        let color = Color::RGB(1, 2, 3);
        assert_eq!(format!("{}", color), "#010203");
    }

    #[test]
    fn test_from_str_hex_valid() {
        let color = Color::from_str("#abc").unwrap();
        assert_eq!(color, Color::RGB(170, 187, 204));

        let color = Color::from_str("#aabbcc").unwrap();
        assert_eq!(color, Color::RGB(170, 187, 204));
    }

    #[test]
    fn test_from_str_rgb_valid() {
        let color = Color::from_str("rgb(255, 0, 128)").unwrap();
        assert_eq!(color, Color::RGB(255, 0, 128));
    }

    #[test]
    fn test_from_str_ycbcr_valid() {
        let color = Color::from_str("ycbcr(100.0, 100.0, 100.0)").unwrap();
        assert_eq!(color, Color::from_ycbcr(100.0, 100.0, 100.0));
    }

    #[test]
    fn test_from_str_invalid_hex() {
        assert!(Color::from_str("#abcd").is_err());
        assert!(Color::from_str("#xyzxyz").is_err());
    }

    #[test]
    fn test_from_str_invalid_rgb() {
        assert!(Color::from_str("rgb(255, 255)").is_err());
        assert!(Color::from_str("rgb(255, a, 0)").is_err());
    }

    #[test]
    fn test_from_str_invalid_ycbcr() {
        assert!(Color::from_str("ycbcr(100.0, 100.0)").is_err());
        assert!(Color::from_str("ycbcr(x, y, z)").is_err());
    }

    #[test]
    fn test_from_str_unknown_format() {
        assert!(Color::from_str("hsl(100,100,100)").is_err());
    }
}
