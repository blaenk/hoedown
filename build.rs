#![feature(plugin)]
#![feature(os)]
#![feature(path)]
#![feature(io)]
#![feature(collections)]
#![feature(core)]

#![allow(unused_must_use)]
#![allow(dead_code)]

extern crate regex;
#[plugin] #[no_link]
extern crate regex_macros;

use std::str::FromStr;
use std::old_io::process::Command;
use std::old_io::{fs, BufferedReader, File};
use std::fmt;
use std::os;

fn main() {
  let root_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
  let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());

  Command::new("make")
    .arg("-C").arg("libhoedown")
    .arg("libhoedown.a")
    .status().unwrap();

  let lib_path = root_dir.join("libhoedown/libhoedown.a");
  let target = out_dir.join("libhoedown.a");

  fs::rename(&lib_path, &target).unwrap();

  println!("cargo:rustc-flags=-L native={} -l static=hoedown", out_dir.as_str().unwrap());
}

fn code_gen(root_dir: &Path, out_dir: &Path) {
  // generate hoedown callback infrastructure
  let header_path = root_dir.join("libhoedown/src/document.h");
  let mut header = BufferedReader::new(File::open(&header_path));

  // extern callback signatures
  let callback_types_path = out_dir.join("callback_types.rs");
  let mut callback_types_file = File::create(&callback_types_path);

  // closure types
  let closure_types_path = out_dir.join("closure_types.rs");
  let mut closure_types_file = File::create(&closure_types_path);

  // register callbacks
  let register_callbacks_path = out_dir.join("register_callbacks.rs");
  let mut register_callbacks_file = File::create(&register_callbacks_path);

  // wrappers
  let wrappers_path = out_dir.join("wrappers.rs");
  let mut wrappers_file = File::create(&wrappers_path);

  // render functions
  let render_functions_path = out_dir.join("render_functions.rs");
  let mut render_functions_file = File::create(&render_functions_path);

  render_functions_file.write_line("impl Render for Html {");
  render_functions_file.write_line(
"
fn to_hoedown(&mut self) -> hoedown_renderer {
    unsafe { *self.renderer.0 }
}");

  // renderer callbacks struct
  let renderer_callbacks_path = out_dir.join("renderer_callbacks.rs");
  let mut renderer_callbacks_file = File::create(&renderer_callbacks_path);

  renderer_callbacks_file.write_line("#[derive(Copy)]");
  renderer_callbacks_file.write_line("#[repr(C)]");
  renderer_callbacks_file.write_line("pub struct hoedown_renderer {");
  renderer_callbacks_file.write_line("pub opaque: *mut c_void,");

  // renderer callbacks struct
  let hoedown_renderer_path = out_dir.join("hoedown_renderer.rs");
  let mut hoedown_renderer_file = File::create(&hoedown_renderer_path);

  hoedown_renderer_file.write_line(
"
fn hoedown_renderer_for_trait<R>(renderer: &mut R) -> hoedown_renderer
where R: Render {
    use libc::c_void;

    hoedown_renderer {
        opaque: renderer as *mut _ as *mut c_void,");

  // renderer callbacks struct
  let closures_struct_path = out_dir.join("closures_struct.rs");
  let mut closures_struct_file = File::create(&closures_struct_path);

  closures_struct_file.write_line("pub struct Closures<'a> {");

  // renderer trait
  let trait_path = out_dir.join("trait.rs");
  let mut trait_file = File::create(&trait_path);

  trait_file.write_line("pub trait Render: Sized {");
  trait_file.write_line(
"fn to_hoedown(&mut self) -> hoedown_renderer {
    hoedown_renderer_for_trait(self)
}");

  let functions: Vec<Function> = header.lines()
      // trim whitespace
      .map(|line| line.unwrap().trim().to_string())
      // ignore empty lines
      .filter(|line| !line.is_empty())
      // don't capture until the struct hoedown_renderer definition begins
      .skip_while(|line| !regex!(r"struct hoedown_renderer \{").is_match(line.as_slice()))
      // skip the first line
      .skip(1)
      // stop capturing once we reach the end of the hoedown_renderer definition
      .take_while(|line| line.as_slice() != "};")
      // ignore comments
      .filter(|line| !line.starts_with("/*"))
      // skip the opaque pointer
      .skip(1)
      // parse them into Functions
      .map(|line| line.parse::<Function>().unwrap())
      .collect();

  for function in functions.iter() {
      callback_types_file.write_line(function.as_extern_signature().as_slice());
      closure_types_file.write_line(function.as_closure_type().as_slice());
      register_callbacks_file.write_line(function.as_register_callback().as_slice());
      wrappers_file.write_line(function.as_wrapper().as_slice());

      hoedown_renderer_file.write_line(function.as_renderer_field().as_slice());

      render_functions_file.write_line(function.as_render_function().as_slice());
      renderer_callbacks_file.write_line(function.as_callback_field().as_slice());
      closures_struct_file.write_line(function.as_closures_field().as_slice());
      trait_file.write_line(function.as_trait_method().as_slice());
  }

  hoedown_renderer_file.write_line("}}");
  trait_file.write_line("}");
  render_functions_file.write_line("}");
  renderer_callbacks_file.write_line("}");
  closures_struct_file.write_line("}\n");

  renderer_callbacks_file.write_line(
"//impl hoedown_renderer {
    // pub fn merge(left: hoedown_renderer, right: hoedown_renderer)
    // -> hoedown_renderer {
        // hoedown_renderer {
            // opaque: left.opaque,");

  closures_struct_file.write_line("impl <'a> Closures<'a> {");
  closures_struct_file.write_line("pub fn new() -> Closures<'a> {");
  closures_struct_file.write_line("Closures {");

  for function in functions.iter() {
      closures_struct_file.write_line(function.as_closures_field_new().as_slice());
      // renderer_callbacks_file.write_line(
      //     format!("{name}: left.{name}.or(right.{name}),", name = function.name).as_slice());
  }

  // renderer_callbacks_file.write_line("}");
  // renderer_callbacks_file.write_line("}");
  // renderer_callbacks_file.write_line("}");

  closures_struct_file.write_line("}");
  closures_struct_file.write_line("}");
  closures_struct_file.write_line("}");

  closures_struct_file.write_line("impl<'a> Render for Closures<'a> {");

  for function in functions.iter() {
      closures_struct_file.write_line(function.as_closure_function().as_slice());
  }

  closures_struct_file.write_line("}");
}

enum Ty {
    Void,
    Buffer,
    Int,
    UInt,
    ListFlags,
    TableFlags,
    AutoLinkFlags,
}

impl Ty {
    fn is_void(&self) -> bool {
        if let &Ty::Void = self { true } else { false }
    }

    fn is_int(&self) -> bool {
        if let &Ty::Int = self { true } else { false }
    }

    fn is_uint(&self) -> bool {
        if let &Ty::UInt = self { true } else { false }
    }

    fn ffi(&self) -> &'static str {
        match *self {
            Ty::Void => "c_void",
            Ty::Buffer => "hoedown_buffer",
            Ty::Int => "c_int",
            Ty::UInt => "c_uint",
            Ty::ListFlags => "::renderer::list::Flags",
            Ty::TableFlags => "::renderer::Table",
            Ty::AutoLinkFlags => "::renderer::AutoLink",
        }
    }

    fn rust(&self) -> &'static str {
        match *self {
            Ty::Void => "()",
            Ty::Buffer => "Buffer",
            Ty::Int => "i32",
            Ty::UInt => "u32",
            Ty::ListFlags => "::renderer::list::Flags",
            Ty::TableFlags => "::renderer::Table",
            Ty::AutoLinkFlags => "::renderer::AutoLink",
        }
    }
}

impl FromStr for Ty {
    type Err = String;

    fn from_str(s: &str) -> Result<Ty, String> {
        match s {
            "hoedown_renderer_data" | "void" => Ok(Ty::Void),
            "hoedown_buffer" => Ok(Ty::Buffer),
            "hoedown_list_flags" => Ok(Ty::ListFlags),
            "hoedown_table_flags" => Ok(Ty::TableFlags),
            "hoedown_autolink_type" => Ok(Ty::AutoLinkFlags),
            "int" => Ok(Ty::Int),
            "unsigned int" => Ok(Ty::UInt),
            _ => Err("Parser error".to_string()),
        }
    }
}

enum Pointer {
    None,
    Const,
    Mut,
}

// maybe replace this with clang bindings when they exist
struct Argument  {
    pointer_type: Pointer,
    argument_type: Ty,
    name: String,
}

impl Argument {
    fn ffi_type(&self) -> String {
        let mut ty = String::new();

        ty.push_str(match self.pointer_type {
            Pointer::Const => "*const ",
            Pointer::Mut => "*mut ",
            Pointer::None => "",
        });

        ty.push_str(self.argument_type.ffi());

        ty
    }

    fn rust_type(&self) -> String {
        let mut ty = String::new();
        let ctype = self.argument_type.rust();

        if ctype == "c_void" {
            ty.push_str("*mut c_void");
        } else {
            ty.push_str(match self.pointer_type {
                Pointer::Const => "&",
                Pointer::Mut => "&mut ",
                Pointer::None => "",
            });

            ty.push_str(ctype);
        }

        ty
    }

    fn ffi_parameter(&self) -> String {
        format!("{}: {}", self.name, self.ffi_type())
    }

    fn rust_parameter(&self) -> String {
        format!("{}: {}", self.name, self.rust_type())
    }

    fn to_wrapper_parameter(&self) -> String {
        if let Ty::Buffer = self.argument_type {
            match self.pointer_type {
                Pointer::Const => format!("&Buffer::from({} as *mut _)", self.name),
                Pointer::Mut => format!("&mut Buffer::from({})", self.name),
                _ => unreachable!(),
            }
        } else {
            self.name.clone()
        }
    }

    fn to_extern_parameter(&self) -> String {
        if let Ty::Buffer = self.argument_type {
            match self.pointer_type {
                Pointer::Const => format!("{}.get()", self.name),
                Pointer::Mut => format!("{}.get_mut()", self.name),
                _ => unreachable!(),
            }
        } else {
            if self.argument_type.is_void() {
                "::std::ptr::null_mut::<c_void>()".to_string()
            } else {
                self.castable()
            }
        }
    }

    fn castable(&self) -> String {
        if self.argument_type.is_int() || self.argument_type.is_uint() {
            format!("{} as {}", self.name.as_slice(), self.argument_type.rust())
        } else {
            self.name.clone()
        }
    }
}

impl FromStr for Argument {
    type Err = String;

    fn from_str(s: &str) -> Result<Argument, String> {
        let mut ty = String::new();
        let mut name = String::new();

        // hoedown_buffer *ob
        // const hoedown_buffer *ob
        // int level
        let chunks = regex!(r"(?P<is_const>const )?(?P<argument_type>.+) (?P<is_ptr>\*)?(?P<name>\w+)");
        let cap = chunks.captures(s.as_slice()).unwrap();
        let mut ptrty = Pointer::None;

        ty.push_str(cap.name("argument_type").unwrap());

        if cap.name("argument_type").unwrap() == "hoedown_renderer_data" {
            ptrty = Pointer::Mut;
        } else if cap.name("is_ptr").is_some() {
            if cap.name("is_const").is_some() {
                ptrty = Pointer::Const;
            } else {
                ptrty = Pointer::Mut;
            }
        }

        let mut n = cap.name("name").unwrap();

        // avoid keywords
        if n == "type" {
            n = "ty";
        }

        name.push_str(n);

        Ok(
            Argument {
                pointer_type: ptrty,
                argument_type: ty.parse::<Ty>().unwrap(),
                name: name.to_string()
            }
        )
    }
}

impl fmt::Debug for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.rust_parameter())
    }
}

struct Function {
    return_type: Ty,
    name: String,
    arguments: Vec<Argument>,
}

impl Function {
    fn rust_types(&self) -> String {
        let raws: Vec<String> =
            self.arguments.iter()
                .filter(|arg| !arg.argument_type.is_void())
                .map(|arg| arg.rust_type())
                .collect();

        raws.as_slice().connect(", ")
    }

    fn rust_parameters(&self) -> String {
        let raws: Vec<String> =
            self.arguments.iter()
                .filter(|arg| !arg.argument_type.is_void())
                .map(|arg| arg.rust_parameter()).collect();

        raws.as_slice().connect(", ")
    }

    fn as_extern_signature(&self) -> String {
        let raws: Vec<String> = self.arguments.iter().map(|arg| arg.ffi_type()).collect();
        let connected = raws.as_slice().connect(", ");

        format!(r#"pub type {name} = extern "C" fn({arguments}) -> {return_type};"#,
            name = self.name,
            arguments = connected,
            return_type = self.return_type.rust())
    }

    fn as_callback_field(&self) -> String {
        format!("pub {name}: Option<callbacks::{name}>,", name = self.name)
    }

    fn as_closures_field(&self) -> String {
        format!("{name}: Option<types::{name}<'a>>,", name = self.name)
    }

    fn as_renderer_field(&self) -> String {
        format!("{name}: Some(wrappers::{name}::<R>),", name = self.name)
    }

    fn as_closures_field_new(&self) -> String {
        format!("{name}: None,", name = self.name)
    }

    fn as_wrapper(&self) -> String {
        let raws: Vec<String> = self.arguments.iter().map(|arg| arg.ffi_parameter()).collect();
        let extern_args = raws.as_slice().connect(", ");

        let raws: Vec<String> =
            self.arguments.iter()
                .filter(|arg| !arg.argument_type.is_void())
                .map(|arg| arg.to_wrapper_parameter())
                .collect();

        let wrapper_parameters = raws.as_slice().connect(", ");

        format!(
r#"
pub extern "C" fn {name}<R>({extern_arguments}) -> {return_type}
where R: Render {{
    let renderer = data as *mut hoedown_renderer;
    let renderer = unsafe {{ (*renderer).opaque as *mut R}};
    let renderer = unsafe {{ &mut *renderer }};
    renderer.{name}({wrapper_parameters});
    {return_value}
}}"#,
            name = self.name,
            extern_arguments = extern_args,
            wrapper_parameters = wrapper_parameters,
            return_type = self.return_type.rust(),
            return_value = if self.return_type.is_int() { "1" } else { "()" })
    }

    fn as_render_function(&self) -> String {
        let rust_arguments = self.rust_parameters();

        let raws: Vec<String> =
            self.arguments.iter()
                .filter(|arg| arg.name.as_slice() != "data")
                .map(|arg| arg.to_extern_parameter())
                .collect();

        let extern_parameters = raws.as_slice().connect(", ");

        format!(
"
fn {name}(&mut self, {rust_arguments}) {{
    let data = unsafe {{ self.renderer.0 as *mut c_void }};
    let func = unsafe {{ (*self.renderer.0).{name}.unwrap() }};
    func({extern_parameters}, data);
}}",
            name = self.name,
            rust_arguments = rust_arguments,
            extern_parameters = extern_parameters)
    }

    fn as_trait_method(&self) -> String {
        let rust_arguments = self.rust_parameters();

        format!("fn {name}(&mut self, {rust_arguments}) {{}}",
            name = self.name,
            rust_arguments = rust_arguments)
    }

    fn as_closure_function(&self) -> String {
        let rust_arguments = self.rust_parameters();

        let raws: Vec<String> =
            self.arguments.iter()
                .filter(|arg| !arg.argument_type.is_void())
                .map(|arg| arg.castable())
                .collect();

        let extern_parameters = raws.as_slice().connect(", ");

        format!(
r#"
fn {name}(&mut self, {rust_arguments}) {{
    if let Some(ref mut func) = self.{name} {{
        func({extern_parameters});
    }}
}}"#,
            name = self.name,
            rust_arguments = rust_arguments,
            extern_parameters = extern_parameters)
    }

    fn as_closure_type(&self) -> String {
        let connected = self.rust_types();

        format!(r#"pub type {name}<'a> = Box<FnMut({arguments}) + 'a>;"#,
            name = self.name,
            arguments = connected)
    }

    fn as_register_callback(&self) -> String {
        let connected = self.rust_types();

        format!(
"
impl<'a> Closures<'a> {{
    pub fn on_{name}<F>(&mut self, closure: F)
    where F: FnMut({arguments}), F: 'a {{
        self.{name} = Some(Box::new(closure));
    }}
}}",
            name = self.name,
            arguments = connected)
    }
}

impl FromStr for Function {
    type Err = String;

    fn from_str(s: &str) -> Result<Function, String> {
        // void (*hrule)(hoedown_buffer *ob, const hoedown_renderer_data *data);
        let chunks = regex!(r"(?P<return_type>\w+) \(\*?(?P<name>\w+)\)\((?P<arguments>.+)\);");
        let cap = chunks.captures(s.as_slice()).unwrap();

        let name = cap.name("name").unwrap();
        let arguments: Vec<Argument> =
            cap.name("arguments").unwrap()
            .split_str(", ")
            .map(|argument| argument.parse::<Argument>().unwrap())
            .collect();
        let return_type = cap.name("return_type").unwrap();

        Ok(
            Function {
                return_type: return_type.parse::<Ty>().unwrap(),
                name: name.to_string(),
                arguments: arguments,
            }
        )
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "name: {}", self.name);
        writeln!(f, "arguments: {:?}", self.arguments);
        writeln!(f, "return_type: {}", self.return_type.rust())
    }
}

