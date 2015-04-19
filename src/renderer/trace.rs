use std::io::Write;

use buffer::Buffer;
use super::Render;

/// This renderer implements the block handlers
/// to make it easy to determine which handlers need to be implemented
/// for a given document.
pub struct Trace;

#[allow(unused_variables)]
impl Render for Trace {
    fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
        output.write(b"MISSING CODE_BLOCK HANDLER\n").unwrap();
    }

    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING QUOTE_BLOCK HANDLER\n").unwrap();
    }

    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        output.write(b"MISSING HEADER HANDLER\n").unwrap();
    }

    fn horizontal_rule(&mut self, output: &mut Buffer) {
        output.write(b"MISSING HORIZONTAL_RULE HANDLER\n").unwrap();
    }

    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        output.write(b"MISSING LIST HANDLER\n").unwrap();
    }

    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        output.write(b"MISSING LIST_ITEM HANDLER\n").unwrap();
    }

    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING PARAGRAPH HANDLER\n").unwrap();
    }

    fn table(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE HANDLER\n").unwrap();
    }

    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE_HEADER HANDLER\n").unwrap();
    }

    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE_BODY HANDLER\n").unwrap();
    }

    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE_ROW HANDLER\n").unwrap();
    }

    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
        output.write(b"MISSING TABLE_CELL HANDLER\n").unwrap();
    }

    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING FOOTNOTES HANDLER\n").unwrap();
    }

    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
        output.write(b"MISSING FOOTNOTE_DEFINITION HANDLER\n").unwrap();
    }

    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
        output.write(b"MISSING HTML_BLOCK HANDLER\n").unwrap();
    }
}
