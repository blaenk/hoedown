use std::io::{self, Read, Write};

use buffer::Buffer;
use renderer::Render;
use document::Document;
use extensions::Extension;

/// Markdown document
pub struct Markdown {
    contents: Buffer,
    extensions: Extension,
    max_nesting: usize,
}

impl Markdown {
    pub fn new(body: &str) -> Markdown {
        Markdown::from(body.as_bytes())
    }

    /// Construct a markdown document from a given Reader
    ///
    /// By default it enables no Hoedown extensions and sets the maximum
    /// block depth to parse at 16. This may be changed with the `with_extensions`
    /// and `with_max_nesting` builder methods.
    ///
    /// Note that `Buffer` also implements `Reader`, so it can be used with this
    /// method.
    pub fn read_from<R>(mut reader: R) -> Markdown
    where R: Read {
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();

        Markdown {
            contents: Buffer::from(&contents[..]),
            extensions: Extension::empty(),
            max_nesting: 16,
        }
    }

    /// Builder method to specify Hoedown extensions
    pub fn extensions(mut self, extensions: Extension) -> Markdown {
        self.extensions = extensions;
        self
    }

    /// Builder method to specify the maximum block depth to parse
    pub fn max_nesting(mut self, max_nesting: usize) -> Markdown {
        self.max_nesting = max_nesting;
        self
    }

    /// Render the document into the given buffer
    pub fn render_into_buffer<R>(&self, mut renderer: R, output: &mut Buffer)
    where R: Render {
        let renderer = unsafe { renderer.to_hoedown() };
        let doc = Document::new(&renderer, self.extensions, self.max_nesting);
        doc.render(output, self.contents.as_ref());
    }

    /// Render the document to a buffer that is returned
    pub fn render_to_buffer<R>(&self, renderer: R) -> Buffer
    where R: Render {
        let mut output = Buffer::new(64);
        self.render_into_buffer(renderer, &mut output);
        output
    }

    /// Render the document to a given Write
    pub fn render<R, W>(&self, renderer: R, writer: &mut W) -> io::Result<()>
    where R: Render, W: Write {
        let output = self.render_to_buffer(renderer);
        writer.write_all(&output)
    }

    /// Render the document as inline into the given buffer
    pub fn render_inline_into_buffer<R>(&self, mut renderer: R, output: &mut Buffer)
    where R: Render {
        let renderer = unsafe { renderer.to_hoedown() };
        let doc = Document::new(&renderer, self.extensions, self.max_nesting);
        doc.render_inline(output, self.contents.as_ref());
    }

    /// Render the document as inline to a buffer that is returned
    pub fn render_inline_to_buffer<R>(&self, renderer: R) -> Buffer
    where R: Render {
        let mut output = Buffer::new(64);
        self.render_inline_into_buffer(renderer, &mut output);
        output
    }

    /// Render the document as inline to a given Write
    pub fn render_inline<R, W>(&self, renderer: R, writer: &mut W) -> io::Result<()>
    where R: Render, W: Write {
        let output = self.render_inline_to_buffer(renderer);
        writer.write_all(&output)
    }
}

impl From<Buffer> for Markdown {
    fn from(buffer: Buffer) -> Markdown {
        Markdown {
            contents: buffer,
            extensions: Extension::empty(),
            max_nesting: 16,
        }
    }
}

impl<'a> From<&'a [u8]> for Markdown {
    fn from(bytes: &[u8]) -> Markdown {
        let buffer = Buffer::from(bytes);

        Markdown {
            contents: buffer,
            extensions: Extension::empty(),
            max_nesting: 16,
        }
    }
}


