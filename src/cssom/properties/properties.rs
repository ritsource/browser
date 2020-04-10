#[derive(Eq, PartialEq)]
pub struct Integer(i32);

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Measure {
    Px(MeasureValue),
    Percent(MeasureValue),
    // Rem(MeasureFloat),
    Auto,
}

#[derive(Eq, PartialEq)]
pub struct MeasureValue {
    pub i: i32,
    pub f: i32,
}

#[allow(dead_code)]
impl MeasureValue {
    pub fn new(i: i32, f: i32) -> Self {
        Self { i, f }
    }
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Property {
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    JustifyContent(JustifyContent),
    VerticalAlign(VerticalAlign),
    Background(Color),
    BackgroundColor(Color),
    // ***
    BackgroundImage,
    BorderTopWidth(Measure),
    BorderTopStyle(BorderStyle),
    BorderTopColor(Color),
    BorderBottomWidth(Measure),
    BorderBottomStyle(BorderStyle),
    BorderBottomColor(Color),
    BorderLeftWidth(Measure),
    BorderLeftStyle(BorderStyle),
    BorderLeftColor(Color),
    BorderRightWidth(Measure),
    BorderRightStyle(BorderStyle),
    BorderRightColor(Color),
    BorderTopLeftRadius(Measure),
    BorderTopRightRadius(Measure),
    BorderBottomLeftRadius(Measure),
    BorderBottomRightRadius(Measure),
    Height(Measure),
    MaxHeight(Measure),
    MinHeight(Measure),
    Width(Measure),
    MaxWidth(Measure),
    MinWidth(Measure),
    Top(Measure),
    Bottom(Measure),
    Left(Measure),
    Right(Measure),
    MarginTop(Measure),
    MarginBottom(Measure),
    MarginLeft(Measure),
    MarginRight(Measure),
    PaddingTop(Measure),
    PaddingBottom(Measure),
    PaddingLeft(Measure),
    PaddingRight(Measure),
    Color(Color),
    // Font,
    // FontFamily(Vec<String>),
    FontSize(Measure),
    FontWeight(FontWeight),
    LetterSpacing(Measure),
    LineHeight(Measure),
    TextAlign(TextAlign),
    TextDecoration(TextDecoration),
    Cursor(Cursor),
    Display(Display),
    // FlexBasis(Measure),
    FlexDirection(FlexDirection),
    // FlexFlow(FlexFlow),
    // FlexGrow(FlexGrow),
    // FlexShrink,
    // FlexWrap,
    Opacity(Opacity),
    Visibility(Visibility),
    OutlineWidth(Measure),
    OutlineStyle(OutlineStyle),
    OutlineColor(Color),
    // OutlineOffset(Measure),
    Overflow(Overflow),
    OverflowX(Overflow),
    OverflowY(Overflow),
    Position(Position),
    ZIndex(Integer),
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum AlignContent {
    Normal,
    Center,
    Start,
    End,
    SpaceBetween,
    SpaceAround,
    Baseline,
    Streach,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum AlignItems {
    Normal,
    Center,
    Start,
    End,
    Baseline,
    Streach,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum AlignSelf {
    Auto,
    Normal,
    Center,
    Start,
    End,
    Baseline,
    Streach,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum JustifyContent {
    Normal,
    Center,
    Start,
    End,
    Left,
    Right,
    Streach,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum VerticalAlign {
    Middle,
    Top,
    Bottom,
    Baseline,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum BorderStyle {
    Solid,
    Dotted,
    Dashed,
    None,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Integer(Integer),
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Start,
    End,
    Justify,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum TextDecoration {
    Underline,
    LineThrough,
    None,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Cursor {
    Default,
    Pointer,
    Text,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Display {
    Inline,
    InlineBlock,
    Block,
    Flex,
    None,
}

impl Default for Display {
    fn default() -> Display {
        Display::Inline
    }
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
    Inherit,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Opacity {
    Integer(Integer),
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
    Inherit,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum OutlineStyle {
    Solid,
    Dashed,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Auto,
    Scroll,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Sticky,
    Fixed,
}
