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
//!let doc = Markdown::new("some _emphasis_ required");
//!let html = Html::new(html::Flags::empty(), 0);
//!
//!assert_eq!(
//!    doc.render_to_buffer(html).to_str().unwrap(),
//!    "<p>some <em>emphasis</em> required</p>\n");
//!```

#![feature(unique)]

extern crate libc;

#[macro_use]
extern crate bitflags;

mod extensions;
pub mod ffi;
mod buffer;
pub mod renderer;
mod document;
mod wrappers;
mod markdown;

pub use markdown::Markdown;
pub use buffer::Buffer;
pub use renderer::Render;
pub use extensions::*;

