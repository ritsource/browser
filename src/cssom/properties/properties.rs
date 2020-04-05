// pub enum Property {
//     Overflow,
//     Display,
//     Padding((Measure),
//     Margin((Measure),
// }

// pub enum Measure {
//     pub px(i32),
//     pub percent(i32),
//     pub rem(i32),
// }

#[allow(dead_code)]
pub enum Property {
    Overflow,
    Cursor,
}

#[allow(dead_code)]
pub enum Overflow {
    Auto,
    Hidden,
    Scroll,
    Visible,
}

#[allow(dead_code)]
pub enum Cursor {
    Default,
    Grab,
    Pointer,
}
