use crate::{
    __setter, 
    __xml_test_suites,
    formatting::{BottomBorder, TopBorder},
    private_prelude::*,
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblBorders")]
pub struct TableBorders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomBorder<'a>>,
}

impl<'a> TableBorders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
}

__xml_test_suites!(
    TableBorders,
    TableBorders::default(),
    r#"<w:tblBorders/>"#,
    TableBorders::default().top(TopBorder::default()),
    r#"<w:tblBorders><w:top/></w:tblBorders>"#,
    TableBorders::default().bottom(BottomBorder::default()),
    r#"<w:tblBorders><w:bottom/></w:tblBorders>"#,
);
