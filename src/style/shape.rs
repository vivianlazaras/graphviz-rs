use std::fmt;
use std::str::FromStr;

/// Represents the shape of a node in a graph visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum NodeShape {
    /// A simple rectangle with sharp corners.
    ///
    /// ```none
    /// ┌─────┐
    /// │     │
    /// └─────┘
    /// ```
    Box,

    /// A polygon with a configurable number of sides.
    /// Usually a hexagon or other multi-sided figure.
    ///
    /// ```none
    ///   /‾‾‾\
    ///  /     \
    ///  \     /
    ///   \___/
    /// ```
    Polygon,

    /// An ellipse (oval) shape.
    ///
    /// ```none
    ///   _____
    ///  /     \
    /// |       |
    ///  \_____/
    /// ```
    Ellipse,

    /// A perfect circle.
    ///
    /// ```none
    ///   ___
    ///  /   \
    /// |     |
    ///  \___/
    /// ```
    Circle,

    /// A very small point or dot.
    ///
    /// ```none
    ///  •
    /// ```
    Point,

    /// An egg-shaped oval, wider at one end.
    ///
    /// ```none
    ///   __
    ///  /  \_
    ///  \___/
    /// ```
    Egg,

    /// A triangle with a point at the top.
    ///
    /// ```none
    ///    /\
    ///   /  \
    ///  /____\
    /// ```
    Triangle,

    /// Plain text label, no enclosing shape.
    ///
    /// Just text displayed.
    Plaintext,

    /// A diamond shape.
    ///
    /// ```none
    ///    /\
    ///   /  \
    ///   \  /
    ///    \/
    /// ```
    Diamond,

    /// A trapezium (trapezoid) shape, with one pair of parallel sides.
    ///
    /// ```none
    ///   ______
    ///  /      \
    ///  \______/
    /// ```
    Trapezium,

    /// A parallelogram shape, slanted rectangle.
    ///
    /// ```none
    ///    ______
    ///   /      /
    ///  /______/
    /// ```
    Parallelogram,

    /// A house-like shape with a triangular roof and rectangular base.
    ///
    /// ```none
    ///    /\
    ///   /  \
    ///  |____|
    /// ```
    House,

    /// A pentagon with five sides.
    ///
    /// ```none
    ///    /‾‾\
    ///   /    \
    ///   \    /
    ///    \__/
    /// ```
    Pentagon,

    /// A hexagon with six sides.
    ///
    /// ```none
    ///   ____
    ///  /    \
    ///  \____/
    /// ```
    Hexagon,

    /// A septagon with seven sides.
    /// Similar to hexagon but with an additional side.
    Septagon,

    /// An octagon with eight sides.
    ///
    /// ```none
    ///   ______
    ///  /      \
    ///  \______/
    /// ```
    Octagon,

    /// A double circle — one circle inside another.
    ///
    /// ```none
    ///   _____
    ///  /     \
    /// |  o o  |
    ///  \_____/
    /// ```
    DoubleCircle,

    /// A double octagon — an octagon within an octagon.
    DoubleOctagon,

    /// A triple octagon — three nested octagons.
    TripleOctagon,

    /// An inverted triangle (point down).
    ///
    /// ```none
    ///  \    /
    ///   \  /
    ///    \/
    /// ```
    InvTriangle,

    /// An inverted trapezium (flipped trapezoid).
    InvTrapezium,

    /// An inverted house shape (point down roof).
    InvHouse,

    /// A diamond with hollow center (medial diamond).
    Mdiamond,

    /// A square with hollow center (medial square).
    Msquare,

    /// A circle with hollow center (medial circle).
    Mcircle,

    /// A rectangle shape (same as Box but usually different rendering).
    Rect,

    /// Same as Rect.
    Rectangle,

    /// A perfect square.
    Square,

    /// A star shape with five or more points.
    ///
    /// ```none
    ///    *
    ///  *   *
    ///    *
    /// ```
    Star,

    /// No shape (invisible).
    None,

    /// A note or sticky note shape (a rectangle with a folded corner).
    Note,

    /// A tab shape (like a folder tab).
    Tab,

    /// A folder shape.
    Folder,

    /// A 3D box shape (a cube drawn in 3D perspective).
    Box3d,

    /// A component shape (usually a rectangle with little side bars).
    Component,

    /// A cylinder shape (for databases, pipes).
    Cylinder,

    /// A record shape (like a table or struct with fields).
    Record,
}

impl fmt::Display for NodeShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NodeShape::*;
        let s = match self {
            Box => "box",
            Polygon => "polygon",
            Ellipse => "ellipse",
            Circle => "circle",
            Point => "point",
            Egg => "egg",
            Triangle => "triangle",
            Plaintext => "plaintext",
            Diamond => "diamond",
            Trapezium => "trapezium",
            Parallelogram => "parallelogram",
            House => "house",
            Pentagon => "pentagon",
            Hexagon => "hexagon",
            Septagon => "septagon",
            Octagon => "octagon",
            DoubleCircle => "doublecircle",
            DoubleOctagon => "doubleoctagon",
            TripleOctagon => "tripleoctagon",
            InvTriangle => "invtriangle",
            InvTrapezium => "invtrapezium",
            InvHouse => "invhouse",
            Mdiamond => "Mdiamond",
            Msquare => "Msquare",
            Mcircle => "Mcircle",
            Rect => "rect",
            Rectangle => "rectangle",
            Square => "square",
            Star => "star",
            None => "none",
            Note => "note",
            Tab => "tab",
            Folder => "folder",
            Box3d => "box3d",
            Component => "component",
            Cylinder => "cylinder",
            Record => "record",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for NodeShape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NodeShape::*;
        match s.to_lowercase().as_str() {
            "box" => Ok(Box),
            "polygon" => Ok(Polygon),
            "ellipse" => Ok(Ellipse),
            "circle" => Ok(Circle),
            "point" => Ok(Point),
            "egg" => Ok(Egg),
            "triangle" => Ok(Triangle),
            "plaintext" => Ok(Plaintext),
            "diamond" => Ok(Diamond),
            "trapezium" => Ok(Trapezium),
            "parallelogram" => Ok(Parallelogram),
            "house" => Ok(House),
            "pentagon" => Ok(Pentagon),
            "hexagon" => Ok(Hexagon),
            "septagon" => Ok(Septagon),
            "octagon" => Ok(Octagon),
            "doublecircle" => Ok(DoubleCircle),
            "doubleoctagon" => Ok(DoubleOctagon),
            "tripleoctagon" => Ok(TripleOctagon),
            "invtriangle" => Ok(InvTriangle),
            "invtrapezium" => Ok(InvTrapezium),
            "invhouse" => Ok(InvHouse),
            "mdiamond" => Ok(Mdiamond),
            "msquare" => Ok(Msquare),
            "mcircle" => Ok(Mcircle),
            "rect" => Ok(Rect),
            "rectangle" => Ok(Rectangle),
            "square" => Ok(Square),
            "star" => Ok(Star),
            "none" => Ok(None),
            "note" => Ok(Note),
            "tab" => Ok(Tab),
            "folder" => Ok(Folder),
            "box3d" => Ok(Box3d),
            "component" => Ok(Component),
            "cylinder" => Ok(Cylinder),
            "record" => Ok(Record),
            _ => Err("Unsupported shape"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum ArrowType {
    Normal,
    Vee,
    Tee,
    Dot,
    Odot,
    Diamond,
    Odiamond,
    Box,
    Obox,
    Crow,
    Icurve,
    Inv,
    None,
    Empty,
    Halfopen,
    Open,
    NormalOpen,
}

impl fmt::Display for ArrowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ArrowType::Normal => "normal",
            ArrowType::Vee => "vee",
            ArrowType::Tee => "tee",
            ArrowType::Dot => "dot",
            ArrowType::Odot => "odot",
            ArrowType::Diamond => "diamond",
            ArrowType::Odiamond => "odiamond",
            ArrowType::Box => "box",
            ArrowType::Obox => "obox",
            ArrowType::Crow => "crow",
            ArrowType::Icurve => "icurve",
            ArrowType::Inv => "inv",
            ArrowType::None => "none",
            ArrowType::Empty => "empty",
            ArrowType::Halfopen => "halfopen",
            ArrowType::Open => "open",
            ArrowType::NormalOpen => "normalopen",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ArrowType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(ArrowType::Normal),
            "vee" => Ok(ArrowType::Vee),
            "tee" => Ok(ArrowType::Tee),
            "dot" => Ok(ArrowType::Dot),
            "odot" => Ok(ArrowType::Odot),
            "diamond" => Ok(ArrowType::Diamond),
            "odiamond" => Ok(ArrowType::Odiamond),
            "box" => Ok(ArrowType::Box),
            "obox" => Ok(ArrowType::Obox),
            "crow" => Ok(ArrowType::Crow),
            "icurve" => Ok(ArrowType::Icurve),
            "inv" => Ok(ArrowType::Inv),
            "none" => Ok(ArrowType::None),
            "empty" => Ok(ArrowType::Empty),
            "halfopen" => Ok(ArrowType::Halfopen),
            "open" => Ok(ArrowType::Open),
            "normalopen" => Ok(ArrowType::NormalOpen),
            _ => Err("Invalid arrow type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_display_all_variants() {
        let cases = vec![
            (ArrowType::Normal, "normal"),
            (ArrowType::Vee, "vee"),
            (ArrowType::Tee, "tee"),
            (ArrowType::Dot, "dot"),
            (ArrowType::Odot, "odot"),
            (ArrowType::Diamond, "diamond"),
            (ArrowType::Odiamond, "odiamond"),
            (ArrowType::Box, "box"),
            (ArrowType::Obox, "obox"),
            (ArrowType::Crow, "crow"),
            (ArrowType::Icurve, "icurve"),
            (ArrowType::Inv, "inv"),
            (ArrowType::None, "none"),
            (ArrowType::Empty, "empty"),
            (ArrowType::Halfopen, "halfopen"),
            (ArrowType::Open, "open"),
            (ArrowType::NormalOpen, "normalopen"),
        ];

        for (arrow_type, expected_str) in cases {
            assert_eq!(arrow_type.to_string(), expected_str);
        }
    }

    #[test]
    fn test_from_str_all_variants() {
        let cases = vec![
            ("normal", ArrowType::Normal),
            ("vee", ArrowType::Vee),
            ("tee", ArrowType::Tee),
            ("dot", ArrowType::Dot),
            ("odot", ArrowType::Odot),
            ("diamond", ArrowType::Diamond),
            ("odiamond", ArrowType::Odiamond),
            ("box", ArrowType::Box),
            ("obox", ArrowType::Obox),
            ("crow", ArrowType::Crow),
            ("icurve", ArrowType::Icurve),
            ("inv", ArrowType::Inv),
            ("none", ArrowType::None),
            ("empty", ArrowType::Empty),
            ("halfopen", ArrowType::Halfopen),
            ("open", ArrowType::Open),
            ("normalopen", ArrowType::NormalOpen),
        ];

        for (input, expected) in cases {
            assert_eq!(ArrowType::from_str(input).unwrap(), expected);
        }
    }

    #[test]
    fn arrow_type_test_from_str_case_insensitive() {
        assert_eq!(ArrowType::from_str("NoRmAl").unwrap(), ArrowType::Normal);
        assert_eq!(ArrowType::from_str("DIAMOND").unwrap(), ArrowType::Diamond);
        assert_eq!(ArrowType::from_str("oBoX").unwrap(), ArrowType::Obox);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(ArrowType::from_str("invalid").is_err());
        assert!(ArrowType::from_str("").is_err());
        assert!(ArrowType::from_str("123").is_err());
    }

    #[test]
    fn test_display_all_shapes() {
        let cases = vec![
            (NodeShape::Box, "box"),
            (NodeShape::Polygon, "polygon"),
            (NodeShape::Ellipse, "ellipse"),
            (NodeShape::Circle, "circle"),
            (NodeShape::Point, "point"),
            (NodeShape::Egg, "egg"),
            (NodeShape::Triangle, "triangle"),
            (NodeShape::Plaintext, "plaintext"),
            (NodeShape::Diamond, "diamond"),
            (NodeShape::Trapezium, "trapezium"),
            (NodeShape::Parallelogram, "parallelogram"),
            (NodeShape::House, "house"),
            (NodeShape::Pentagon, "pentagon"),
            (NodeShape::Hexagon, "hexagon"),
            (NodeShape::Septagon, "septagon"),
            (NodeShape::Octagon, "octagon"),
            (NodeShape::DoubleCircle, "doublecircle"),
            (NodeShape::DoubleOctagon, "doubleoctagon"),
            (NodeShape::TripleOctagon, "tripleoctagon"),
            (NodeShape::InvTriangle, "invtriangle"),
            (NodeShape::InvTrapezium, "invtrapezium"),
            (NodeShape::InvHouse, "invhouse"),
            (NodeShape::Mdiamond, "Mdiamond"),
            (NodeShape::Msquare, "Msquare"),
            (NodeShape::Mcircle, "Mcircle"),
            (NodeShape::Rect, "rect"),
            (NodeShape::Rectangle, "rectangle"),
            (NodeShape::Square, "square"),
            (NodeShape::Star, "star"),
            (NodeShape::None, "none"),
            (NodeShape::Note, "note"),
            (NodeShape::Tab, "tab"),
            (NodeShape::Folder, "folder"),
            (NodeShape::Box3d, "box3d"),
            (NodeShape::Component, "component"),
            (NodeShape::Cylinder, "cylinder"),
            (NodeShape::Record, "record"),
        ];

        for (shape, expected_str) in cases {
            assert_eq!(shape.to_string(), expected_str);
        }
    }

    #[test]
    fn test_from_str_all_shapes() {
        let cases = vec![
            ("box", NodeShape::Box),
            ("polygon", NodeShape::Polygon),
            ("ellipse", NodeShape::Ellipse),
            ("circle", NodeShape::Circle),
            ("point", NodeShape::Point),
            ("egg", NodeShape::Egg),
            ("triangle", NodeShape::Triangle),
            ("plaintext", NodeShape::Plaintext),
            ("diamond", NodeShape::Diamond),
            ("trapezium", NodeShape::Trapezium),
            ("parallelogram", NodeShape::Parallelogram),
            ("house", NodeShape::House),
            ("pentagon", NodeShape::Pentagon),
            ("hexagon", NodeShape::Hexagon),
            ("septagon", NodeShape::Septagon),
            ("octagon", NodeShape::Octagon),
            ("doublecircle", NodeShape::DoubleCircle),
            ("doubleoctagon", NodeShape::DoubleOctagon),
            ("tripleoctagon", NodeShape::TripleOctagon),
            ("invtriangle", NodeShape::InvTriangle),
            ("invtrapezium", NodeShape::InvTrapezium),
            ("invhouse", NodeShape::InvHouse),
            ("mdiamond", NodeShape::Mdiamond),
            ("msquare", NodeShape::Msquare),
            ("mcircle", NodeShape::Mcircle),
            ("rect", NodeShape::Rect),
            ("rectangle", NodeShape::Rectangle),
            ("square", NodeShape::Square),
            ("star", NodeShape::Star),
            ("none", NodeShape::None),
            ("note", NodeShape::Note),
            ("tab", NodeShape::Tab),
            ("folder", NodeShape::Folder),
            ("box3d", NodeShape::Box3d),
            ("component", NodeShape::Component),
            ("cylinder", NodeShape::Cylinder),
            ("record", NodeShape::Record),
        ];

        for (input, expected) in cases {
            assert_eq!(
                NodeShape::from_str(input).unwrap(),
                expected,
                "failed to parse: {}",
                input
            );
        }
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(NodeShape::from_str("BoX").unwrap(), NodeShape::Box);
        assert_eq!(
            NodeShape::from_str("RECTANGLE").unwrap(),
            NodeShape::Rectangle
        );
        assert_eq!(NodeShape::from_str("cIrClE").unwrap(), NodeShape::Circle);
    }

    #[test]
    fn test_from_str_invalid_shape() {
        assert!(NodeShape::from_str("hexadecagon").is_err());
        assert!(NodeShape::from_str("notashape").is_err());
        assert!(NodeShape::from_str("").is_err());
    }
}
