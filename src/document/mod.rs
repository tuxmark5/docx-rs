//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.

mod body;
mod bookmark_end;
mod bookmark_start;
mod r#break;
mod comment_range;
mod drawing;
mod grid_column;
mod hyperlink;
mod paragraph;
mod run;
mod smart_tag;
mod tab;
mod table;
mod table_cell;
mod table_grid;
mod table_row;
mod text;

pub use self::{
    body::*, 
    bookmark_end::*, 
    bookmark_start::*, 
    comment_range::*, 
    drawing::*,
    grid_column::*, 
    hyperlink::*, 
    paragraph::*,
    r#break::*, 
    run::*, 
    smart_tag::SmartTag,
    table::*, 
    table_cell::*, 
    table_grid::*, 
    table_row::*, 
    text::*,
};

use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;

use crate::__xml_test_suites;
use crate::schema::SCHEMA_MAIN;

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:document")]
pub struct Document<'a> {
    /// Specifies the body of the docment.
    #[xml(child = "w:body")]
    pub body: Body<'a>,
}

impl<'a> Document<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.body.push(content);
        self
    }
}

impl<'a> XmlWrite for Document<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Document { body } = self;

        log::debug!("[Document] Started writing.");

        writer.write_element_start("w:document")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_element_end_open()?;

        body.to_writer(writer)?;

        writer.write_element_end_close("w:document")?;

        log::debug!("[Document] Finished writing.");

        Ok(())
    }
}

__xml_test_suites!(
    Document,
    Document::default(),
    format!(
        r#"<w:document xmlns:w="{}"><w:body/></w:document>"#,
        SCHEMA_MAIN
    )
    .as_str(),
);
