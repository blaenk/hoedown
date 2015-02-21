#![feature(plugin)]
#![feature(env)]
#![feature(fs)]
#![feature(path)]
#![feature(process)]

#![allow(unused_must_use)]
#![allow(dead_code)]

#![plugin(regex_macros)]

extern crate regex;

use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
  let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
  let root_dir = Path::new(&manifest);

  let out = env::var("OUT_DIR").unwrap();
  let out_dir = Path::new(&out);

  Command::new("make")
    .arg("-C").arg("libhoedown")
    .arg("libhoedown.a")
    .status().unwrap();

  let lib_path = root_dir.join("libhoedown/libhoedown.a");
  let target = out_dir.join("libhoedown.a");

  fs::rename(&lib_path, &target).unwrap();

  println!("cargo:rustc-flags=-L native={} -l static=hoedown", out_dir.to_str().unwrap());
}

