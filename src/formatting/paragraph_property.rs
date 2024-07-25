use crate::{
    __setter, 
    __xml_test_suites,
    formatting::{Borders, Justification, NumberingProperty},
    private_prelude::*,
};

/// Paragraph Property
///
/// ```rust
/// use docx::formatting::{ParagraphProperty, JustificationVal};
///
/// let prop = ParagraphProperty::default()
///     .style_id("foo")
///     .justification(JustificationVal::Start)
///     .numbering((10usize, 20usize));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pPr")]
pub struct ParagraphProperty<'a> {
    /// Specifies the style ID of the paragraph style.
    #[xml(child = "w:pStyle")]
    pub style_id: Option<ParagraphStyleId<'a>>,

    /// Specifies the paragraph alignment.
    #[xml(child = "w:jc")]
    pub justification: Option<Justification>,

    /// Specifies borders for the paragraph.
    #[xml(child = "w:pBdr")]
    pub border: Option<Borders<'a>>,

    /// Specifies that the paragraph should be numbered.
    #[xml(child = "w:numPr")]
    pub numbering: Option<NumberingProperty>,

    /// Specifies the outline level
    #[xml(child = "w:outlineLvl")]
    pub outline_level: Option<OutlineLevel>,

    /// Specifies section properties
    #[xml(child = "w:sectPr")]
    pub section: Option<SectionProperty>,
}

impl<'a> ParagraphProperty<'a> {
    __setter!(style_id: Option<ParagraphStyleId<'a>>);
    __setter!(justification: Option<Justification>);
    __setter!(border: Option<Borders<'a>>);
    __setter!(numbering: Option<NumberingProperty>);
}

#[derive(Clone, Copy, Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:outlineLvl")]
pub struct OutlineLevel {
    #[xml(attr = "w:val")]
    pub value: usize,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:sectPr")]
pub struct SectionProperty { }

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pStyle")]
pub struct ParagraphStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for ParagraphStyleId<'a> {
    fn from(val: T) -> Self {
        ParagraphStyleId { value: val.into() }
    }
}

#[cfg(test)]
use crate::formatting::JustificationVal;

__xml_test_suites!(
    ParagraphProperty,
    ParagraphProperty::default(),
    r#"<w:pPr/>"#,
    ParagraphProperty::default().style_id("id"),
    r#"<w:pPr><w:pStyle w:val="id"/></w:pPr>"#,
    ParagraphProperty::default().justification(JustificationVal::Start),
    r#"<w:pPr><w:jc w:val="start"/></w:pPr>"#,
    ParagraphProperty::default().border(Borders::default()),
    r#"<w:pPr><w:pBdr/></w:pPr>"#,
    ParagraphProperty::default().numbering(NumberingProperty::default()),
    r#"<w:pPr><w:numPr><w:numId w:val="0"/><w:ilvl w:val="0"/></w:numPr></w:pPr>"#,
);
