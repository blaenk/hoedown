//! Markdown buffers

use std::old_io::IoResult;
use libc::size_t;
use std::str;
use std::slice;
use std::mem;
use std::ptr::Unique;

use ffi::{
    hoedown_buffer,
    hoedown_buffer_new,
    hoedown_buffer_put,
    hoedown_buffer_free
};

/// Buffer for holding markdown contents
pub struct Buffer {
    buffer: Unique<hoedown_buffer>,
    is_owned: bool,
}

impl Buffer {
    /// Create a buffer with the specified unit allocation size.
    ///
    /// The unit allocation size is the amount by which buffers will
    /// grow as more space is required.
    pub fn new(size: usize) -> Buffer {
        Buffer {
            buffer: unsafe { Unique::new(hoedown_buffer_new(size as size_t)) },
            is_owned: true,
        }
    }

    /// Create a buffer from a string
    pub fn from_str(s: &str) -> Buffer {
        let mut buffer = Buffer::new(64);
        buffer.write_str(s).unwrap();
        buffer
    }

    /// Create a 'read-only' buffer from the given `hoedown_buffer`
    ///
    /// The returned buffer won't take ownership of the passed `hoedown_buffer`,
    /// that is, the returned buffer won't free the underlying buffer
    pub unsafe fn from(buffer: *mut hoedown_buffer) -> Buffer {
        assert!(!buffer.is_null());

        Buffer {
            buffer: Unique::new(buffer),
            is_owned: false,
        }
    }

    /// Get a reference to the underlying buffer
    pub fn get<'a>(&'a self) -> &'a hoedown_buffer {
        unsafe { & **self.buffer }
    }

    /// Get a mutable reference to the underlying buffer
    pub fn get_mut<'a>(&'a mut self) -> &'a mut hoedown_buffer {
        unsafe { &mut **self.buffer }
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The length of the contents inside the buffer
    pub fn len(&self) -> u64 {
        unsafe { (**self.buffer).size }
    }

    /// Get a raw constant pointer to the buffer data
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { (**self.buffer).data }
    }

    /// Pipe another buffer's contents into this one
    pub fn pipe(&mut self, input: &Buffer) {
        unsafe {
            hoedown_buffer_put(*self.buffer, input.as_ptr(), input.len());
        }
    }

    /// Get a slice of the buffer's contents
    pub fn as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let data = (**self.buffer).data as *const u8;
            let size = (**self.buffer).size as usize;

            mem::transmute(slice::from_raw_parts(data, size))
        }
    }

    /// Get a mutable slice of the buffer's contents
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            let data = (**self.buffer).data;
            let size = (**self.buffer).size as usize;

            slice::from_raw_parts_mut(data, size)
        }
    }

    /// Attempt to get a string from the buffer's contents
    pub fn as_str<'a>(&'a self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_slice())
    }
}

unsafe impl Sync for Buffer {}
unsafe impl Send for Buffer {}

impl Clone for Buffer {
    fn clone(&self) -> Buffer {
        // create a buffer with the same unit size
        let unit = unsafe { (**self.buffer).unit };
        let mut buffer = Buffer::new(unit as usize);
        // pipe this one's contents into it
        buffer.pipe(self);
        buffer
    }
}

impl Reader for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        Reader::read(&mut self.as_slice(), buf)
    }
}

impl Writer for Buffer {
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        unsafe {
            hoedown_buffer_put(*self.buffer, buf.as_ptr(), buf.len() as size_t);
        }
        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.is_owned {
            unsafe { hoedown_buffer_free(*self.buffer); }
        }
    }
}

