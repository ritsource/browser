#[allow(dead_code)]
pub enum ElemType {
    H1,
    H2,
    P,
    A,
    Span,
    Html,
    Head,
    Body,
    Style,
    Img,
    Video,
    Canvas,
    Script,
    Div,
    Nav,
    Header,
    Footer,
    Button,
    Form,
    Label,
    Input,
    Meta,
    Link,
    UNSUPPORTED,
}

#[allow(dead_code)]
impl ElemType {
    pub fn from(tag: &str) -> ElemType {
        use ElemType::*;

        match tag {
            "h1" => H1,
            "h2" => H2,
            "p" => P,
            "a" => A,
            "span" => Span,
            "html" => Html,
            "head" => Head,
            "body" => Body,
            "style" => Style,
            "img" => Img,
            "video" => Video,
            "canvas" => Canvas,
            "script" => Script,
            "div" => Div,
            "nav" => Nav,
            "header" => Header,
            "footer" => Footer,
            "button" => Button,
            "from" => Form,
            "label" => Label,
            "input" => Input,
            "meta" => Meta,
            "link" => Link,
            _ => UNSUPPORTED,
        }
    }
}
