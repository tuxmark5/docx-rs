use crate::{
    __setter, 
    __xml_test_suites, 
    formatting::BorderStyle,
    private_prelude::*,
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:bottom")]
pub struct BottomBorder<'a> {
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

impl<'a> BottomBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    BottomBorder,
    BottomBorder::default(),
    r#"<w:bottom/>"#,
    BottomBorder::default().color("000000"),
    r#"<w:bottom w:color="000000"/>"#,
    BottomBorder::default().shadow(false),
    r#"<w:bottom w:shadow="false"/>"#,
    BottomBorder::default().space(40usize),
    r#"<w:bottom w:space="40"/>"#,
    BottomBorder::default().size(20usize),
    r#"<w:bottom w:sz="20"/>"#,
    BottomBorder::default().style(BorderStyle::Dotted),
    r#"<w:bottom w:val="dotted"/>"#,
);
