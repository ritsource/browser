#[allow(dead_code)]
enum X {
    H1(Text),
    H2(Text),
    F1(Fook),
    F2(Fook),
}

struct Text();
struct Fook();

#[allow(dead_code)]
pub enum Tags {
    H1(String),
    H2(String),
    P(String),
    A(String),
    Span(String),
    Html(String),
    Head(String),
    Body(String),
    Style(String),
    Img(String),
    Video(String),
    Canvas(String),
    Script(String),
    Article(String),
    Content(String),
    Div(String),
    Nav(String),
    Header(String),
    Footer(String),
    Button(String),
    Form(String),
    Label(String),
    Input(String),
}

#[allow(dead_code)]
impl Tags {
    pub fn get_new_elem(tag: &str) -> Tags {
        use Tags::*;

        match tag {
            "h1" => H1(String::from("")),
            "h2" => H2(String::from("")),
            "p" => P(String::from("")),
            "a" => A(String::from("")),
            "span" => Span(String::from("")),
            "html" => Html(String::from("")),
            "head" => Head(String::from("")),
            "body" => Body(String::from("")),
            "style" => Style(String::from("")),
            "img" => Img(String::from("")),
            "video" => Video(String::from("")),
            "canvas" => Canvas(String::from("")),
            "script" => Script(String::from("")),
            "div" => Div(String::from("")),
            "nav" => Nav(String::from("")),
            "header" => Header(String::from("")),
            "footer" => Footer(String::from("")),
            "button" => Button(String::from("")),
            "from" => Form(String::from("")),
            "label" => Label(String::from("")),
            "input" => Input(String::from("")),
            _ => Div(String::from("")),
        }
    }
}
