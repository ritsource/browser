pub struct Integer(i32);

pub enum Measure {
    Px(f64),
    Percent(f64),
    // Rem(f64),
    Auto,
}

pub struct Color(String);

impl Color {
    pub fn to_hex(self) -> String {
        self.0
    }
}

#[allow(dead_code)]
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
    FontStyle(FontStyle),
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

pub enum AlignContent {
    F,
}

pub enum AlignItems {
    F,
}

pub enum AlignSelf {
    F,
}

pub enum JustifyContent {
    F,
}

pub enum VerticalAlign {
    F,
}

pub enum Background {
    F,
}

pub enum BackgroundColor {
    F,
}

pub enum BackgroundImage {
    F,
}

pub enum BorderStyle {
    F,
}

pub enum BorderTopWidth {
    F,
}

pub enum BorderTopStyle {
    F,
}

pub enum BorderTopColor {
    F,
}

pub enum BorderBottomWidth {
    F,
}

pub enum BorderBottomStyle {
    F,
}

pub enum BorderBottomColor {
    F,
}

pub enum BorderLeftWidth {
    F,
}

pub enum BorderLeftStyle {
    F,
}

pub enum BorderLeftColor {
    F,
}

pub enum BorderRightWidth {
    F,
}

pub enum BorderRightStyle {
    F,
}

pub enum BorderRightColor {
    F,
}

pub enum BorderTopLeftRadius {
    F,
}

pub enum BorderTopRightRadius {
    F,
}

pub enum BorderBottomLeftRadius {
    F,
}

pub enum BorderBottomRightRadius {
    F,
}

pub enum Height {
    F,
}

pub enum MaxHeight {
    F,
}

pub enum MinHeight {
    F,
}

pub enum MaxWidth {
    F,
}

pub enum MinWidth {
    F,
}

pub enum Bottom {
    F,
}

pub enum MarginTop {
    F,
}

pub enum MarginBottom {
    F,
}

pub enum MarginLeft {
    F,
}

pub enum MarginRight {
    F,
}

pub enum PaddingTop {
    F,
}

pub enum PaddingBottom {
    F,
}

pub enum PaddingLeft {
    F,
}

pub enum PaddingRight {
    F,
}

pub enum FontFamily {
    F,
}

pub enum FontSize {
    F,
}

pub enum FontStyle {
    F,
}

pub enum FontWeight {
    F,
}

pub enum LetterSpacing {
    F,
}

pub enum LineHeight {
    F,
}

pub enum TextAlign {
    F,
}

pub enum TextDecoration {
    F,
}

pub enum Cursor {
    F,
}

pub enum Display {
    F,
}

pub enum FlexBasis {
    F,
}

pub enum FlexDirection {
    F,
}

pub enum FlexFlow {
    F,
}

pub enum FlexGrow {
    F,
}

pub enum FlexShrink {
    F,
}

pub enum FlexWrap {
    F,
}

pub enum Opacity {
    F,
}

pub enum Visibility {
    F,
}

pub enum OutlineWidth {
    F,
}

pub enum OutlineStyle {
    F,
}

pub enum OutlineColor {
    F,
}

pub enum OutlineOffset {
    F,
}

pub enum Overflow {
    F,
}

pub enum OverflowX {
    F,
}

pub enum OverflowY {
    F,
}

pub enum Position {
    F,
}

pub enum ZIndex {
    F,
}
