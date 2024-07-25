use crate::{
    __setter, 
    __xml_test_suites, 
    formatting::BorderStyle,
    private_prelude::*,
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:right")]
pub struct RightBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

impl<'a> RightBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    RightBorder,
    RightBorder::default(),
    r#"<w:right/>"#,
    RightBorder::default().color("000000"),
    r#"<w:right w:color="000000"/>"#,
    RightBorder::default().shadow(false),
    r#"<w:right w:shadow="false"/>"#,
    RightBorder::default().space(40usize),
    r#"<w:right w:space="40"/>"#,
    RightBorder::default().size(20usize),
    r#"<w:right w:sz="20"/>"#,
    RightBorder::default().style(BorderStyle::Dotted),
    r#"<w:right w:val="dotted"/>"#,
);
