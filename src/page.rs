use crate::constants::{PAGE_SIZE, USER_NAME_SIZE};
use crate::row::Row;

pub struct Page {
    pub data: Box<[u8; PAGE_SIZE]>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            data: Box::new([0; PAGE_SIZE]),
        }
    }

    pub fn write_int(&mut self, value: i32, offset: usize) -> usize {
        let bytes = value.to_be_bytes();
        self.data[offset..offset + 4].copy_from_slice(&bytes);
        offset + 4
    }

    pub fn write_bytes<const SIZE: usize>(&mut self, value: &[u8; SIZE], offset: usize) -> usize {
        self.data[offset..offset + SIZE].copy_from_slice(value);
        offset + SIZE
    }

    pub fn read_int(&self, offset: usize) -> i32 {
        i32::from_be_bytes(self.data[offset..offset + 4].try_into().unwrap())
    }

    pub fn read_bytes<const SIZE: usize>(&self, offset: usize) -> Box<[u8; SIZE]> {
        let mut boxed_array = Box::new([0; SIZE]);
        boxed_array.copy_from_slice(&self.data[offset..offset + SIZE]);
        boxed_array
    }

    /// Inserts a compacted repr of row into the page.
    /// Current implementation keeps it very simple.
    pub fn insert_row(&mut self, row: &Row, mut offset: usize) {
        offset = self.write_int(row.id, offset);
        offset = self.write_bytes(&row.username, offset);
        self.write_bytes(&row.email, offset);
    }

    /// Fetch and uncompact row from the page
    pub fn read_row(&self, mut offset: usize) -> Row {
        let id = self.read_int(offset);
        offset += 4;

        let username = self.read_bytes(offset);
        offset += USER_NAME_SIZE;

        let email = self.read_bytes(offset);

        Row {
            id,
            username,
            email,
        }
    }
}
