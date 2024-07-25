use crate::{
    __string_enum, 
    __xml_test_suites,
    private_prelude::*,
};

/// Justification
///
/// ```rust
/// use docx::formatting::*;
///
/// let jc = Justification::from(JustificationVal::Start);
/// ```
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:jc")]
pub struct Justification {
    #[xml(attr = "w:val")]
    pub value: JustificationVal,
}

impl From<JustificationVal> for Justification {
    fn from(value: JustificationVal) -> Self {
        Justification { value }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum JustificationVal {
    Both,
    Center,
    Distribute,
    End,
    HighKashida,
    Left,
    LowKashida,
    MediumKashida,
    NumTab,
    Right,
    Start,
    ThaiDistribute,
}

__string_enum! {
    JustificationVal {
        Both = "both",
        Center = "center",
        Distribute = "distribute",
        End = "end",
        HighKashida = "highKashida",
        Left = "left",
        LowKashida = "lowKashida",
        MediumKashida = "mediumKashida",
        NumTab = "numTab",
        Right = "right",
        Start = "start",
        ThaiDistribute = "thaiDistribute",
    }
}

__xml_test_suites!(
    Justification,
    Justification::from(JustificationVal::Start),
    r#"<w:jc w:val="start"/>"#,
);
