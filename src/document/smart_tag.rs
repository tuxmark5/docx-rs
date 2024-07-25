use crate::{
    document::ParagraphContent,
    private_prelude::*,
};

/// SmartTag
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:smartTag")]
pub struct SmartTag<'a> {
    /// Specifies the content of a run
    #[xml(
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:r",
        child = "w:hyperlink",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:smartTag"
    )]
    pub content: Vec<ParagraphContent<'a>>,
}
