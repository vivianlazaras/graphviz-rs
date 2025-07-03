/// provides custom color parsing, include YCbCr, hex, and RGB
pub mod color;
/// Defines supported shapes for graphviz.
pub mod shape;
use crate::style::shape::{ArrowType, NodeShape};
use color::Color;
use std::ffi::CString;
use std::fmt;
use std::str::FromStr;

fn parse_key_value<'r>(s: &'r str) -> Option<(&'r str, &'r str)> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 { return None; }
    let key = parts[0].trim();
    let mut value = parts[1].trim();
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        value = &value[1..value.len()-1];
    }
    Some((key, value))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum NodeStyle {
    Dashed,
    Dotted,
    Solid,
    Invis,
    Bold,
    Filled,
    Striped,
    Wedged,
    Diagonals,
    Rounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum EdgeStyle {
    Dashed,
    Dotted,
    Solid,
    Invis,
    Bold,
    Tapered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum ClusterStyle {
    Filled,
    Striped,
    Rounded,
}

macro_rules! impl_display_fromstr_lower {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    $(Self::$variant => stringify!($variant),)*
                };
                write!(f, "{}", s.to_ascii_lowercase())
            }
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        x if x.eq_ignore_ascii_case(stringify!($variant)) => Ok(Self::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}

impl_display_fromstr_lower!(NodeStyle {
    Dashed, Dotted, Solid, Invis, Bold, Filled, Striped, Wedged, Diagonals, Rounded
});

impl_display_fromstr_lower!(EdgeStyle {
    Dashed, Dotted, Solid, Invis, Bold, Tapered
});

impl_display_fromstr_lower!(ClusterStyle {
    Filled, Striped, Rounded
});

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum NodeAttribute {
    Common(CommonAttr),
    NodeAttr(NodeAttr)
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum EdgeAttribute {
    Common(CommonAttr),
    EdgeAttr(EdgeAttr),
}

impl FromStr for NodeAttribute {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(common) = s.parse::<CommonAttr>() {
            Ok(NodeAttribute::Common(common))
        } else if let Ok(node_attr) = s.parse::<NodeAttr>() {
            Ok(NodeAttribute::NodeAttr(node_attr))
        } else {
            Err("Failed to parse NodeAttribute")
        }
    }
}

impl FromStr for EdgeAttribute {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(common) = s.parse::<CommonAttr>() {
            Ok(EdgeAttribute::Common(common))
        } else if let Ok(edge_attr) = s.parse::<EdgeAttr>() {
            Ok(EdgeAttribute::EdgeAttr(edge_attr))
        } else {
            Err("Failed to parse EdgeAttribute")
        }
    }
}

/// Provides a trait for all attribute types that can be converted into Graphviz-compatible key-value pairs.
pub trait Attribute {
    /// Convert attribute enum to (name, value) as CString pairs
    /// This is useful for passing to native graphviz FFI
    fn to_cstrings(&self) -> (CString, CString);
    fn default(&self) -> &'static str {
        ""
    }
}

/// Direction of rank layout in Graphviz (`rankdir` attribute).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum RankDir {
    /// Top to Bottom
    TB,
    /// Bottom to Top
    BT,
    /// Left to Right
    LR,
    /// Right to Left
    RL,
}

impl fmt::Display for RankDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RankDir::TB => "TB",
            RankDir::BT => "BT",
            RankDir::LR => "LR",
            RankDir::RL => "RL",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for RankDir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TB" => Ok(RankDir::TB),
            "BT" => Ok(RankDir::BT),
            "LR" => Ok(RankDir::LR),
            "RL" => Ok(RankDir::RL),
            _ => Err("Invalid rankdir value"),
        }
    }
}

/// Direction for edges (`dir` attribute).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum Direction {
    /// Normal arrow direction (forward)
    Forward,
    /// Reverse arrow direction (back)
    Back,
    /// Arrow on both ends
    Both,
    /// No arrows
    None,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::Forward => "forward",
            Direction::Back => "back",
            Direction::Both => "both",
            Direction::None => "none",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "back" => Ok(Direction::Back),
            "both" => Ok(Direction::Both),
            "none" => Ok(Direction::None),
            _ => Err("Invalid direction value"),
        }
    }
}

/// Vertical position of node labels (`labelloc` attribute).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum LabelLoc {
    Top,
    Center,
    Bottom,
}

impl FromStr for LabelLoc {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "t" => Ok(LabelLoc::Top),
            "c" => Ok(LabelLoc::Center),
            "b" => Ok(LabelLoc::Bottom),
            _ => Err("Invalid LabelLoc value"),
        }
    }
}

impl std::fmt::Display for LabelLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LabelLoc::Top => "t",
            LabelLoc::Center => "c",
            LabelLoc::Bottom => "b",
        };
        write!(f, "{}", s)
    }
}

/// Wrapper enum to categorize all supported Graphviz attribute types.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum GraphvizAttr {
    /// Common attributes (used across nodes, edges, and graphs)
    Common(CommonAttr),
    /// Node-specific attributes
    Node(NodeAttr),
    /// Edge specific attributes
    Edge(EdgeAttr),
    /// Graph specific attributes
    Graph(GraphAttr),
}

/// Attributes specific to nodes.
#[derive(Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum NodeAttr {
    /// Shape of the node (e.g. ellipse, box)
    Shape(NodeShape),
    Style(NodeStyle),
    Color(Color),
    FillColor(Color),
    LabelLoc(LabelLoc),
    Width(f32),
    Height(f32),
    FixedSize(bool),
    Image(String),
    Peripheries(u32),
}

impl Attribute for NodeAttr {
    fn to_cstrings(&self) -> (CString, CString) {
        self.to_cstrings()
    }
}

impl NodeAttr {
    pub fn to_cstrings(&self) -> (CString, CString) {
        use NodeAttr::*;
        match self {
            Shape(v) => (
                CString::new("shape").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            Height(v) => (
                CString::new("height").unwrap(),
                CString::new(format!("{:.3}", v)).unwrap(),
            ),
            Width(v) => (
                CString::new("width").unwrap(),
                CString::new(format!("{:.3}", v)).unwrap(),
            ),
            FixedSize(v) => (
                CString::new("fixedsize").unwrap(),
                CString::new(if *v { "true" } else { "false" }).unwrap(),
            ),
            FillColor(c) => (
                CString::new("fillcolor").unwrap(),
                CString::new(c.to_string()).unwrap(),
            ),
            Color(c) => (
                CString::new("color").unwrap(),
                CString::new(c.to_string()).unwrap(),
            ),
            Image(i) => (
                CString::new("image").unwrap(),
                CString::new(i.to_string()).unwrap(),
            ),
            Peripheries(v) => (
                CString::new("peripheries").unwrap(),
                CString::new(format!("{:.3}", v)).unwrap(),
            ),
            Style(s) => (
                CString::new("style").unwrap(),
                CString::new(s.to_string()).unwrap(),
            ),
            LabelLoc(l) => (
                CString::new("labelloc").unwrap(),
                CString::new(l.to_string()).unwrap(),
            ),
        }
    }
}

impl std::str::FromStr for NodeAttr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = parse_key_value(s) {
            match key {
                "shape" => value.parse().map(NodeAttr::Shape).map_err(|_| "Invalid shape"),
                "style" => value.parse().map(NodeAttr::Style).map_err(|_| "Invalid style"),
                "color" => value.parse().map(NodeAttr::Color).map_err(|_| "Invalid color"),
                "fillcolor" => value.parse().map(NodeAttr::FillColor).map_err(|_| "Invalid color"),
                "labelloc" => value.parse().map(NodeAttr::LabelLoc).map_err(|_| "Invalid labelloc"),
                "width" => value.parse().map(NodeAttr::Width).map_err(|_| "Invalid width"),
                "height" => value.parse().map(NodeAttr::Height).map_err(|_| "Invalid height"),
                "fixedsize" => {
                    match value {
                        "true" => Ok(NodeAttr::FixedSize(true)),
                        "false" => Ok(NodeAttr::FixedSize(false)),
                        _ => Err("Invalid fixedsize")
                    }
                },
                "image" => Ok(NodeAttr::Image(value.to_string())),
                "peripheries" => value.parse().map(NodeAttr::Peripheries).map_err(|_| "Invalid peripheries"),
                _ => Err("Unknown NodeAttr key"),
            }
        } else {
            Err("Invalid format")
        }
    }
}

impl fmt::Debug for NodeAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for NodeAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NodeAttr::*;
        match self {
            Shape(v) => write!(f, "shape=\"{}\"", v),
            Style(v) => write!(f, "style=\"{}\"", v),
            Color(v) => write!(f, "color=\"{}\"", v),
            FillColor(v) => write!(f, "fillcolor=\"{}\"", v),
            LabelLoc(v) => write!(f, "labelloc=\"{}\"", v),
            Width(v) => write!(f, "width=\"{}\"", v),
            Height(v) => write!(f, "height=\"{}\"", v),
            FixedSize(v) => write!(f, "fixedsize=\"{}\"", v),
            Image(v) => write!(f, "image=\"{}\"", v),
            Peripheries(v) => write!(f, "peripheries=\"{}\"", v),
        }
    }
}

/// Attributes specific to edges.
#[derive(Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum EdgeAttr {
    ArrowHead(ArrowType),
    ArrowTail(ArrowType),
    Dir(Direction),
    Weight(f32),
    MinLen(u32),
    LabelDistance(f32),
    LabelAngle(f32),
    Constraint(bool),
    Style(EdgeStyle),
    Color(Color),
}

impl Attribute for EdgeAttr {
    fn to_cstrings(&self) -> (CString, CString) {
        self.to_cstrings()
    }
}

impl EdgeAttr {
    pub fn to_cstrings(&self) -> (CString, CString) {
        use EdgeAttr::*;
        match self {
            ArrowHead(v) => (
                CString::new("arrowhead").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            Weight(v) => (
                CString::new("weight").unwrap(),
                CString::new(format!("{}", v)).unwrap(),
            ),
            Constraint(v) => (
                CString::new("constraint").unwrap(),
                CString::new(if *v { "true" } else { "false" }).unwrap(),
            ),
            Color(c) => (
                CString::new("color").unwrap(),
                CString::new(c.to_string()).unwrap(),
            ),
            ArrowTail(v) => (
                CString::new("arrowtail").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            Dir(v) => (
                CString::new("dir").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            MinLen(v) => (
                CString::new("minlen").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            LabelDistance(v) => (
                CString::new("labeldistance").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            LabelAngle(v) => (
                CString::new("labelangle").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            Style(v) => (
                CString::new("style").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
        }
    }
}

impl std::str::FromStr for EdgeAttr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = parse_key_value(s) {
            match key {
                "arrowhead" => value.parse().map(EdgeAttr::ArrowHead).map_err(|_| "Invalid arrowhead"),
                "arrowtail" => value.parse().map(EdgeAttr::ArrowTail).map_err(|_| "Invalid arrowtail"),
                "dir" => value.parse().map(EdgeAttr::Dir).map_err(|_| "Invalid dir"),
                "weight" => value.parse().map(EdgeAttr::Weight).map_err(|_| "Invalid weight"),
                "minlen" => value.parse().map(EdgeAttr::MinLen).map_err(|_| "Invalid minlen"),
                "labeldistance" => value.parse().map(EdgeAttr::LabelDistance).map_err(|_| "Invalid labeldistance"),
                "labelangle" => value.parse().map(EdgeAttr::LabelAngle).map_err(|_| "Invalid labelangle"),
                "constraint" => {
                    match value {
                        "true" => Ok(EdgeAttr::Constraint(true)),
                        "false" => Ok(EdgeAttr::Constraint(false)),
                        _ => Err("Invalid constraint")
                    }
                },
                "style" => value.parse().map(EdgeAttr::Style).map_err(|_| "Invalid style"),
                "color" => value.parse().map(EdgeAttr::Color).map_err(|_| "Invalid color"),
                _ => Err("Unknown EdgeAttr key"),
            }
        } else {
            Err("Invalid format")
        }
    }
}

impl fmt::Debug for EdgeAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for EdgeAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EdgeAttr::*;
        match self {
            ArrowHead(v) => write!(f, "arrowhead=\"{}\"", v),
            ArrowTail(v) => write!(f, "arrowtail=\"{}\"", v),
            Dir(v) => write!(f, "dir=\"{}\"", v),
            Weight(v) => write!(f, "weight=\"{}\"", v),
            MinLen(v) => write!(f, "minlen=\"{}\"", v),
            LabelDistance(v) => write!(f, "labeldistance=\"{}\"", v),
            LabelAngle(v) => write!(f, "labelangle=\"{}\"", v),
            Constraint(v) => write!(f, "constraint=\"{}\"", v),
            Style(v) => write!(f, "style=\"{}\"", v),
            Color(v) => write!(f, "color=\"{}\"", v),
        }
    }
}

/// Attributes specific to the entire graph.
#[derive(Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum GraphAttr {
    Layout(crate::Layout),
    RankDir(RankDir),
    BgColor(Color),
    Center(bool),
    DPI(f32),
    Margin(f32),
    NodeSep(f32),
    RankSep(f32),
    Size((f32, f32)), // width, height
}

impl Attribute for GraphAttr {
    fn to_cstrings(&self) -> (CString, CString) {
        self.to_cstrings()
    }
}

impl GraphAttr {
    pub fn to_cstrings(&self) -> (CString, CString) {
        use GraphAttr::*;
        match self {
            Layout(v) => (
                CString::new("layout").unwrap(),
                CString::new(v.to_string()).unwrap(),
            ),
            Center(v) => (
                CString::new("center").unwrap(),
                CString::new(if *v { "true" } else { "false" }).unwrap(),
            ),
            DPI(v) => (
                CString::new("dpi").unwrap(),
                CString::new(format!("{}", v)).unwrap(),
            ),
            Size((w, h)) => (
                CString::new("size").unwrap(),
                CString::new(format!("{:.2},{:.2}", w, h)).unwrap(),
            ),
            RankDir(r) => (
                CString::new("rankdir").unwrap(),
                CString::new(r.to_string()).unwrap(),
            ),
            BgColor(c) => (
                CString::new("bgcolor").unwrap(),
                CString::new(c.to_string()).unwrap(),
            ),
            Margin(m) => (
                CString::new("margin").unwrap(),
                CString::new(m.to_string()).unwrap(),
            ),
            NodeSep(n) => (
                CString::new("nodesep").unwrap(),
                CString::new(n.to_string()).unwrap(),
            ),
            RankSep(r) => (
                CString::new("ranksep").unwrap(),
                CString::new(r.to_string()).unwrap(),
            ),
        }
    }
}

impl std::str::FromStr for GraphAttr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = parse_key_value(s) {
            match key {
                "layout" => value.parse().map(GraphAttr::Layout).map_err(|_| "Invalid layout"),
                "rankdir" => value.parse().map(GraphAttr::RankDir).map_err(|_| "Invalid rankdir"),
                "bgcolor" => value.parse().map(GraphAttr::BgColor).map_err(|_| "Invalid color"),
                "center" => {
                    match value {
                        "true" => Ok(GraphAttr::Center(true)),
                        "false" => Ok(GraphAttr::Center(false)),
                        _ => Err("Invalid center value")
                    }
                },
                "dpi" => value.parse().map(GraphAttr::DPI).map_err(|_| "Invalid dpi"),
                "margin" => value.parse().map(GraphAttr::Margin).map_err(|_| "Invalid margin"),
                "nodesep" => value.parse().map(GraphAttr::NodeSep).map_err(|_| "Invalid nodesep"),
                "ranksep" => value.parse().map(GraphAttr::RankSep).map_err(|_| "Invalid ranksep"),
                "size" => {
                    let nums: Vec<&str> = value.split(',').collect();
                    if nums.len() != 2 { return Err("Invalid size"); }
                    let w = nums[0].parse::<f32>().map_err(|_| "Invalid size width")?;
                    let h = nums[1].parse::<f32>().map_err(|_| "Invalid size height")?;
                    Ok(GraphAttr::Size((w, h)))
                }
                _ => Err("Unknown GraphAttr key"),
            }
        } else {
            Err("Invalid format")
        }
    }
}

impl fmt::Debug for GraphAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for GraphAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphAttr::*;
        match self {
            Layout(v) => write!(f, "layout=\"{}\"", v),
            RankDir(v) => write!(f, "rankdir=\"{}\"", v),
            BgColor(v) => write!(f, "bgcolor=\"{}\"", v),
            Center(v) => write!(f, "center=\"{}\"", v),
            DPI(v) => write!(f, "dpi=\"{}\"", v),
            Margin(v) => write!(f, "margin=\"{}\"", v),
            NodeSep(v) => write!(f, "nodesep=\"{}\"", v),
            RankSep(v) => write!(f, "ranksep=\"{}\"", v),
            Size((w, h)) => write!(f, "size=\"{},{}\"", w, h),
        }
    }
}

/// Attributes that are common accross nodes, edges, and graphs
#[derive(Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum CommonAttr {
    Label(String),
    FontSize(f32),
    FontName(String),
    Id(String),
    Class(String),
    Tooltip(String),
    URL(String),
}

impl Attribute for CommonAttr {
    fn to_cstrings(&self) -> (CString, CString) {
        self.to_cstrings()
    }
}

impl CommonAttr {
    pub fn to_cstrings(&self) -> (CString, CString) {
        use CommonAttr::*;
        match self {
            Label(v) => (
                CString::new("label").unwrap(),
                CString::new(v.as_str()).unwrap(),
            ),
            FontSize(v) => (
                CString::new("fontsize").unwrap(),
                CString::new(format!("{}", v)).unwrap(),
            ),
            URL(v) => (
                CString::new("URL").unwrap(),
                CString::new(v.as_str()).unwrap(),
            ),
            Class(c) => (
                CString::new("class").unwrap(),
                CString::new(c.clone()).unwrap(),
            ),
            Id(i) => (
                CString::new("id").unwrap(),
                CString::new(i.clone()).unwrap(),
            ),
            Tooltip(t) => (
                CString::new("tooltip").unwrap(),
                CString::new(t.clone()).unwrap(),
            ),
            FontName(f) => (
                CString::new("fontname").unwrap(),
                CString::new(f.to_string()).unwrap(),
            ),
        }
    }
}

impl fmt::Debug for CommonAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for CommonAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CommonAttr::*;
        match self {
            Label(v) => write!(f, "label=\"{}\"", v),
            FontSize(v) => write!(f, "fontsize=\"{}\"", v),
            FontName(v) => write!(f, "fontname=\"{}\"", v),
            Id(v) => write!(f, "id=\"{}\"", v),
            Class(v) => write!(f, "class=\"{}\"", v),
            Tooltip(v) => write!(f, "tooltip=\"{}\"", v),
            URL(v) => write!(f, "URL=\"{}\"", v),
        }
    }
}

impl std::str::FromStr for CommonAttr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = parse_key_value(s) {
            match key {
                "label" => Ok(CommonAttr::Label(value.to_string())),
                "fontsize" => value.parse().map(CommonAttr::FontSize).map_err(|_| "Invalid fontsize"),
                "fontname" => Ok(CommonAttr::FontName(value.to_string())),
                "id" => Ok(CommonAttr::Id(value.to_string())),
                "class" => Ok(CommonAttr::Class(value.to_string())),
                "tooltip" => Ok(CommonAttr::Tooltip(value.to_string())),
                "url" => Ok(CommonAttr::URL(value.to_string())),
                _ => Err("Unknown CommonAttr key"),
            }
        } else {
            Err("Invalid format")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_common_attr_round_trip() {
        let original = CommonAttr::Label("hello world".into());
        let text = original.to_string();
        let parsed = CommonAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = CommonAttr::FontSize(14.0);
        let text = original.to_string();
        let parsed = CommonAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_node_attr_round_trip() {
        let original = NodeAttr::Shape(NodeShape::Box);
        let text = original.to_string();
        let parsed = NodeAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = NodeAttr::Width(1.5);
        let text = original.to_string();
        let parsed = NodeAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = NodeAttr::FixedSize(true);
        let text = original.to_string();
        let parsed = NodeAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_edge_attr_round_trip() {
        let original = EdgeAttr::ArrowHead(ArrowType::Normal);
        let text = original.to_string();
        let parsed = EdgeAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = EdgeAttr::Constraint(false);
        let text = original.to_string();
        let parsed = EdgeAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = EdgeAttr::Weight(2.0);
        let text = original.to_string();
        let parsed = EdgeAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_graph_attr_round_trip() {
        let original = GraphAttr::RankDir(RankDir::LR);
        let text = original.to_string();
        let parsed = GraphAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = GraphAttr::DPI(96.0);
        let text = original.to_string();
        let parsed = GraphAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = GraphAttr::Center(true);
        let text = original.to_string();
        let parsed = GraphAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);

        let original = GraphAttr::Size((5.0, 4.0));
        let text = original.to_string();
        let parsed = GraphAttr::from_str(&text).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_node_attribute_parse_common_or_node() {
        let s = "label=\"hello\"";
        let parsed: NodeAttribute = s.parse().unwrap();
        if let NodeAttribute::Common(CommonAttr::Label(v)) = parsed {
            assert_eq!(v, "hello");
        } else {
            panic!("Parsed into wrong variant");
        }

        let s = "shape=\"box\"";
        let parsed: NodeAttribute = s.parse().unwrap();
        if let NodeAttribute::NodeAttr(NodeAttr::Shape(NodeShape::Box)) = parsed {
            // ok
        } else {
            panic!("Parsed into wrong variant");
        }
    }

    #[test]
    fn test_edge_attribute_parse_common_or_edge() {
        let s = "label=\"edge label\"";
        let parsed: EdgeAttribute = s.parse().unwrap();
        if let EdgeAttribute::Common(CommonAttr::Label(v)) = parsed {
            assert_eq!(v, "edge label");
        } else {
            panic!("Parsed into wrong variant");
        }

        let s = "arrowhead=\"normal\"";
        let parsed: EdgeAttribute = s.parse().unwrap();
        if let EdgeAttribute::EdgeAttr(EdgeAttr::ArrowHead(ArrowType::Normal)) = parsed {
            // ok
        } else {
            panic!("Parsed into wrong variant");
        }
    }

    #[test]
    fn test_display_debug_match() {
        let attr = NodeAttr::Width(1.23);
        assert_eq!(format!("{}", attr), format!("{:?}", attr));

        let attr = EdgeAttr::Weight(2.0);
        assert_eq!(format!("{}", attr), format!("{:?}", attr));

        let attr = GraphAttr::DPI(72.0);
        assert_eq!(format!("{}", attr), format!("{:?}", attr));

        let attr = CommonAttr::Tooltip("tip".into());
        assert_eq!(format!("{}", attr), format!("{:?}", attr));
    }
}