use libc::size_t;
use std::ptr::Unique;

use extensions::Extension;
use buffer::Buffer;
use ffi::{
    hoedown_document,
    hoedown_renderer,
    hoedown_document_new,
    hoedown_document_render,
    hoedown_document_render_inline,
    hoedown_document_free
};

/// Document parser
pub struct Document {
    document: Unique<hoedown_document>
}

impl Document {
    /// Construct a new document with the given renderer, extensions, and maximum nesting
    ///
    /// `max_nesting` refers to the maximum block depth that should be parsed
    pub fn new(
        renderer: &hoedown_renderer,
        extensions: Extension,
        max_nesting: usize
    ) -> Document {
        let doc = unsafe {
            hoedown_document_new(renderer, extensions.bits(), max_nesting as size_t)
        };

        Document {
            document: unsafe { Unique::new(doc) },
        }
    }

    /// Render a byte slice input into a provided output buffer.
    pub fn render(&self, input: &[u8], output: &mut Buffer) {
        unsafe {
            hoedown_document_render(
                *self.document,
                output.as_mut(),
                input.as_ptr(),
                input.len() as size_t
            );
        }
    }

    /// Render a byte slice input into a provided output buffer as an inline.
    pub fn render_inline(&self, input: &[u8], output: &mut Buffer) {
        unsafe {
            hoedown_document_render_inline(
                *self.document,
                output.as_mut(),
                input.as_ptr(),
                input.len() as size_t
            );
        }
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe { hoedown_document_free(*self.document); }
    }
}

