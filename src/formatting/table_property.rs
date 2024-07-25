use crate::{
    __setter, 
    __xml_test_suites,
    formatting::{TableBorders, TableIndent, TableJustification, TableWidth},
    private_prelude::*,
};

/// Table Property
///
/// ```rust
/// use docx::formatting::*;
///
/// let prop = TableProperty::default()
///     .style_id("foo")
///     .justification(TableJustificationVal::Start)
///     .indent((50, TableIndentUnit::Pct))
///     .width((50, TableWidthUnit::Pct));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblPr")]
pub struct TableProperty<'a> {
    #[xml(child = "w:tblStyle")]
    pub style_id: Option<TableStyleId<'a>>,
    #[xml(child = "w:jc")]
    pub justification: Option<TableJustification>,
    #[xml(child = "w:tblBorders")]
    pub borders: Option<TableBorders<'a>>,
    #[xml(child = "w:tblInd")]
    pub indent: Option<TableIndent>,
    #[xml(child = "w:tblW")]
    pub width: Option<TableWidth>,
}

impl<'a> TableProperty<'a> {
    __setter!(style_id: Option<TableStyleId<'a>>);
    __setter!(justification: Option<TableJustification>);
    __setter!(borders: Option<TableBorders<'a>>);
    __setter!(indent: Option<TableIndent>);
    __setter!(width: Option<TableWidth>);
}

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblStyle")]
pub struct TableStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for TableStyleId<'a> {
    fn from(val: T) -> Self {
        TableStyleId { value: val.into() }
    }
}

__xml_test_suites!(
    TableProperty,
    TableProperty::default(),
    r#"<w:tblPr/>"#,
    TableProperty::default().style_id("id"),
    r#"<w:tblPr><w:tblStyle w:val="id"/></w:tblPr>"#,
    TableProperty::default().justification(crate::formatting::TableJustificationVal::Start),
    r#"<w:tblPr><w:jc w:val="start"/></w:tblPr>"#,
    TableProperty::default().borders(TableBorders::default()),
    r#"<w:tblPr><w:tblBorders/></w:tblPr>"#,
    TableProperty::default().indent(TableIndent::default()),
    r#"<w:tblPr><w:tblInd/></w:tblPr>"#,
    TableProperty::default().width(TableWidth::default()),
    r#"<w:tblPr><w:tblW/></w:tblPr>"#,
);
