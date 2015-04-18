//! Markdown buffers

use libc::size_t;
use std::str;
use std::slice;
use std::mem;
use std::ptr::Unique;
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

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

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The length of the contents inside the buffer
    pub fn len(&self) -> u64 {
        unsafe { self.buffer.get().size }
    }

    /// Get a raw constant pointer to the buffer data
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { self.buffer.get().data }
    }

    /// Pipe another buffer's contents into this one
    pub fn pipe(&mut self, input: &Buffer) {
        unsafe {
            hoedown_buffer_put(*self.buffer, input.as_ptr(), input.len());
        }
    }

    /// Attempt to get a string from the buffer's contents
    pub fn to_str<'a>(&'a self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_ref())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        assert!(!self.buffer.is_null());

        unsafe {
            if self.is_owned && self.buffer.get().unit != 0 {
                hoedown_buffer_free(*self.buffer);
            }
        }
    }
}

unsafe impl Sync for Buffer {}
unsafe impl Send for Buffer {}

impl Clone for Buffer {
    fn clone(&self) -> Buffer {
        // create a buffer with the same unit size
        let unit = unsafe { self.buffer.get().unit };
        let mut buffer = Buffer::new(unit as usize);
        // pipe this one's contents into it
        buffer.pipe(self);
        buffer
    }
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buffer: &[u8] = self.as_ref();
        Read::read(&mut buffer, buf)
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            hoedown_buffer_put(*self.buffer, buf.as_ptr(), buf.len() as size_t);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> From<&'a [u8]> for Buffer {
    /// Create a buffer from bytes
    fn from(s: &[u8]) -> Buffer {
        let mut buffer = Buffer::new(64);
        buffer.write(s).unwrap();
        buffer
    }
}

impl<'a> From<&'a str> for Buffer {
    /// Create a buffer from a string
    fn from(s: &str) -> Buffer {
        Buffer::from(s.as_bytes())
    }
}

impl From<*mut hoedown_buffer> for Buffer {
    /// Create a 'read-only' buffer from the given `hoedown_buffer`
    ///
    /// The returned buffer won't take ownership of the passed `hoedown_buffer`,
    /// that is, the returned buffer won't free the underlying buffer
    fn from(buffer: *mut hoedown_buffer) -> Buffer {
        // this is a simple workaround for hoedown using
        // NULL as an 'empty buffer', we just make an empty
        // buffer and return that
        if buffer.is_null() {
            Buffer::new(0)
        } else{
            Buffer {
                buffer: unsafe { Unique::new(buffer) },
                is_owned: false,
            }
        }
    }
}

/// Dereference to the underlying bytes.
///
/// This is to hook into the automatic dereference coercions system.
impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl AsRef<[u8]> for Buffer {
    /// Get a slice of the buffer's contents
    fn as_ref<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let data = self.buffer.get().data;
            let size = self.buffer.get().size as usize;

            mem::transmute(slice::from_raw_parts(data, size))
        }
    }
}

impl AsMut<[u8]> for Buffer {
    /// Get a mutable slice of the buffer's contents
    fn as_mut<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            let data = self.buffer.get().data;
            let size = self.buffer.get().size as usize;

            slice::from_raw_parts_mut(data, size)
        }
    }
}

impl AsRef<hoedown_buffer> for Buffer {
    /// Get a reference to the underlying buffer
    fn as_ref<'a>(&'a self) -> &'a hoedown_buffer {
        unsafe { self.buffer.get() }
    }
}

impl AsMut<hoedown_buffer> for Buffer {
    /// Get a mutable reference to the underlying buffer
    fn as_mut<'a>(&'a mut self) -> &'a mut hoedown_buffer {
        unsafe { self.buffer.get_mut() }
    }
}
