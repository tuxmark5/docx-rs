use crate::{
    __string_enum, 
    __xml_test_suites,
    private_prelude::*,
};

/// Table Justification
///
/// ```rust
/// use docx::formatting::*;
///
/// let jc = TableJustification::from(TableJustificationVal::Start);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:jc")]
pub struct TableJustification {
    #[xml(attr = "w:val")]
    pub value: Option<TableJustificationVal>,
}

impl From<TableJustificationVal> for TableJustification {
    fn from(val: TableJustificationVal) -> Self {
        TableJustification { value: Some(val) }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableJustificationVal {
    Start,
    End,
    Center,
    Left,
    Right
}

__string_enum! {
    TableJustificationVal {
        Start = "start",
        End = "end",
        Center = "center",
        Left = "left",
        Right = "right",
    }
}

__xml_test_suites!(
    TableJustification,
    TableJustification::default(),
    "<w:jc/>",
    TableJustification::from(TableJustificationVal::Start),
    r#"<w:jc w:val="start"/>"#,
);
