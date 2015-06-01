#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate gcc;

const LIBHOEDOWN_SRC: &'static [&'static str] = &[
  "libhoedown/src/autolink.c",
  "libhoedown/src/buffer.c",
  "libhoedown/src/document.c",
  "libhoedown/src/escape.c",
  "libhoedown/src/html.c",
  "libhoedown/src/html_blocks.c",
  "libhoedown/src/html_smartypants.c",
  "libhoedown/src/stack.c",
  "libhoedown/src/version.c",
];

fn main() {
  gcc::compile_library("libhoedown.a", LIBHOEDOWN_SRC);
}
