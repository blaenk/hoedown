use std::io::Read;

use buffer::Buffer;
use extensions::Extension;

/// Markdown document
#[derive(Clone)]
pub struct Markdown {
    pub contents: Buffer,
    pub extensions: Extension,
    pub max_nesting: usize,
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
    pub fn read_from<R>(reader: R) -> Markdown
    where R: Read {
        Markdown {
            contents: Buffer::read_from(reader),
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


