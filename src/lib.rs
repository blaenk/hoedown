//! This crate provides bindings for the [hoedown] markdown processing library.
//! It has a `Markdown` type which stores markdown text in a `Buffer`. The
//! markdown can then be rendered with any renderer that implements the `Render`
//! trait. The library comes with an `Html` renderer by default.
//!
//![hoedown]: https://github.com/hoedown/hoedown
//!
//!``` rust
//!# use hoedown::Markdown;
//!# use hoedown::renderer::html::{self, Html};
//!let doc = Markdown::new("some _emphasis_ required".as_bytes());
//!let html = Html::new(html::Flags::empty(), 0);
//!
//!assert_eq!(
//!    doc.render_to_buffer(html).as_str().unwrap(),
//!    "<p>some <em>emphasis</em> required</p>\n");
//!```

#![feature(core)]
#![feature(io)]
#![feature(unique)]

extern crate libc;

#[macro_use]
extern crate bitflags;

mod ffi;
pub mod buffer;
pub mod renderer;
mod document;
mod wrappers;

use std::io::{self, Write, Read};

bitflags! {
    #[doc="Constants for the various Hoedown extensions"]
    flags Extension: u32 {
        // block-level

        #[doc="Process table syntax"]
        const TABLES                = 1 << 0,

        #[doc="Process fenced code"]
        const FENCED_CODE           = 1 << 1,

        #[doc="Process footnotes"]
        const FOOTNOTES             = 1 << 2,

        // span-level

        #[doc="Automatically link URLs and emails"]
        const AUTOLINK              = 1 << 3,

        #[doc="
        Enable strikethrough syntax

        e.g. `~~strike one~~`"]
        const STRIKETHROUGH         = 1 << 4,

        #[doc="Perform an underline instead of emphasis"]
        const UNDERLINE             = 1 << 5,

        #[doc="
        Process highlight syntax

        e.g. `==highlight me==`"]
        const HIGHLIGHT             = 1 << 6,

        #[doc="
        Render quotes differently

        example, the html renderer may use the `<q>` tag"]
        const QUOTE                 = 1 << 7,

        #[doc="
        Process superscript syntax

        e.g. `2^3 = 8`"]
        const SUPERSCRIPT           = 1 << 8,

        #[doc="
        Process math syntax

        e.g. `$$x + y = z$$`"]
        const MATH                  = 1 << 9,

        // other flags

        #[doc="
        Don't parse emphasis inside of words

        e.g. `foo_bar_baz` won't emphasize the 'bar'"]
        const NO_INTRA_EMPHASIS     = 1 << 11,

        #[doc="
        Process ATX header syntax

        e.g. `# Topic`"]
        const SPACE_HEADERS         = 1 << 12,

        #[doc="
        Process the single dollar math syntax.

        e.g. `$x + y = 3$`"]
        const MATH_EXPLICIT         = 1 << 13,

        // negative flags

        #[doc="Ignore indented code blocks"]
        const DISABLE_INDENTED_CODE = 1 << 14,
    }
}

/// Markdown document
pub struct Markdown {
    contents: buffer::Buffer,
    extensions: Extension,
    max_nesting: usize,
}

impl Markdown {
    /// Construct a markdown document from a given Reader
    ///
    /// By default it enables no Hoedown extensions and sets the maximum
    /// block depth to parse at 16. This may be changed with the `with_extensions`
    /// and `with_max_nesting` builder methods.
    ///
    /// Note that `Buffer` also implements `Reader`, so it can be used with this
    /// method.
    pub fn new<R>(mut reader: R) -> Markdown where R: Read {
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();
        let mut buffer = buffer::Buffer::new(64);
        buffer.write_all(&contents).unwrap();

        Markdown {
            contents: buffer,
            extensions: Extension::empty(),
            max_nesting: 16,
        }
    }

    /// Construct a markdown document from a string
    pub fn from_str(s: &str) -> Markdown {
        Markdown {
            contents: buffer::Buffer::from_str(s),
            extensions: Extension::empty(),
            max_nesting: 16,
        }
    }

    /// Builder method to specify Hoedown extensions
    pub fn with_extensions(mut self, extensions: Extension) -> Markdown {
        self.extensions = extensions;
        self
    }

    /// Builder method to specify the maximum block depth to parse
    pub fn with_max_nesting(mut self, max_nesting: usize) -> Markdown {
        self.max_nesting = max_nesting;
        self
    }

    /// Render the document into the given buffer
    pub fn render_into_buffer<R>(&self, mut renderer: R, output: &mut buffer::Buffer)
    where R: renderer::Render {
        let renderer = unsafe { renderer.to_hoedown() };
        let doc = document::Document::new(&renderer, self.extensions, self.max_nesting);
        doc.render(output, self.contents.as_slice());
    }

    /// Render the document to a buffer that is returned
    pub fn render_to_buffer<R>(&self, renderer: R) -> buffer::Buffer
    where R: renderer::Render {
        let mut output = buffer::Buffer::new(64);
        self.render_into_buffer(renderer, &mut output);
        output
    }

    /// Render the document to a given Write
    pub fn render<R, W>(&self, renderer: R, writer: &mut W) -> io::Result<()>
    where R: renderer::Render, W: Write {
        let output = self.render_to_buffer(renderer);
        writer.write_all(output.as_slice())
    }

    /// Render the document as inline into the given buffer
    pub fn render_inline_into_buffer<R>(&self, mut renderer: R, output: &mut buffer::Buffer)
    where R: renderer::Render {
        let renderer = unsafe { renderer.to_hoedown() };
        let doc = document::Document::new(&renderer, self.extensions, self.max_nesting);
        doc.render_inline(output, self.contents.as_slice());
    }

    /// Render the document as inline to a buffer that is returned
    pub fn render_inline_to_buffer<R>(&self, renderer: R) -> buffer::Buffer
    where R: renderer::Render {
        let mut output = buffer::Buffer::new(64);
        self.render_inline_into_buffer(renderer, &mut output);
        output
    }

    /// Render the document as inline to a given Write
    pub fn render_inline<R, W>(&self, renderer: R, writer: &mut W) -> io::Result<()>
    where R: renderer::Render, W: Write {
        let output = self.render_inline_to_buffer(renderer);
        writer.write_all(output.as_slice())
    }
}

