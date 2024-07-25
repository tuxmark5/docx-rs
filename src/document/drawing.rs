use std::{
    borrow::Cow,
    io::Write,
};

use hard_xml::{
    xmlparser::{ElementEnd, Token},
    XmlRead, XmlReader, XmlResult, XmlWrite, XmlWriter
};

/// Drawing
#[derive(Debug, Default)]
pub struct Drawing<'a> {
    pub embed: Option<Cow<'a, str>>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl<'a> Drawing<'a> {
    fn consume_any(&mut self, reader: &mut XmlReader<'a>, token: Token<'a>) -> XmlResult<()> {
        match token {
            Token::ElementStart { local, .. } => { 
                if local.as_bytes() == b"blip" {
                    consume(reader, |r, t| self.consume_blip(r, t))?;
                } else if local.as_bytes() == b"ext" {
                    consume(reader, |r, t| self.consume_ext(r, t))?;
                } else {
                    consume(reader, |r, t| self.consume_any(r, t))?;
                }
            }

            _ => {
                // Ignore
            }
        }

        Ok(())
    }

     fn consume_ext(&mut self, reader: &mut XmlReader<'a>, token: Token<'a>) -> XmlResult<()> {
        match token {
            Token::Attribute { local, value, .. } => { 
                match local.as_bytes() {
                    b"cx" => {
                        self.width = value.as_str().parse().ok();
                    }

                    b"cy" => {
                        self.height = value.as_str().parse().ok();
                    }

                    _ => {
                        // Ignore
                    }
                }
            }

            Token::ElementStart { .. } => { 
                consume(reader, |r, t| self.consume_any(r, t))?;
            }

            _ => {
                // Ignore
            }
        }

        Ok(())
    }

    fn consume_blip(&mut self, reader: &mut XmlReader<'a>, token: Token<'a>) -> XmlResult<()> {
        match token {
            Token::Attribute { local, value, .. } => { 
                if local.as_bytes() == b"embed" {
                    self.embed = Some(Cow::Borrowed(value.as_str()));
                }
            }

            Token::ElementStart { .. } => { 
                consume(reader, |r, t| self.consume_blip(r, t))?;
            }

            _ => {
                // Ignore
            }
        }

        Ok(())
    }
}

impl<'a> XmlRead<'a> for Drawing<'a> {
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut this = Self::default();
        consume_root(reader, |r, t| this.consume_any(r, t))?;
        Ok(this)
    }
}

impl<'a> XmlWrite for Drawing<'a> {
    fn to_writer<W: Write>(&self, _writer: &mut XmlWriter<W>) -> XmlResult<()> {
        Ok(())
    }
}

/**************************************************************************************************/

fn consume<'a, F>(reader: &mut XmlReader<'a>, mut f: F) -> XmlResult<()> 
where
    F: FnMut(&mut XmlReader<'a>, Token<'a>) -> XmlResult<()>
{
    loop {
        let token = match reader.next() {
            Some(token) => token?,
            None => break Ok(()),
        };

        match token {
            Token::ElementEnd { end, .. } if is_close(&end) => {
                break Ok(())
            }

            other => {
                f(reader, other)?;
            }
        }
    }
}

fn consume_root<'a, F>(reader: &mut XmlReader<'a>, f: F) -> XmlResult<()> 
where
    F: FnMut(&mut XmlReader<'a>, Token<'a>) -> XmlResult<()>
{
    let token = match reader.next() {
        Some(token) => token?,
        None => return Ok(()),
    };

    if let Token::ElementStart { .. } = token {
        consume(reader, f)
    } else {
        panic!("no element start")
    }
}

fn is_close(end: &ElementEnd) -> bool {
    match end {
        ElementEnd::Open => false,
        ElementEnd::Close(_, _) => true,
        ElementEnd::Empty => true,
    }
}

/**************************************************************************************************/
