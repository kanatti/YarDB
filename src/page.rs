use crate::constants::{EMAIL_SIZE, PAGE_SIZE, USER_NAME_SIZE};
use crate::row::Row;

pub struct Page {
    pub data: Box<[u8; PAGE_SIZE]>,
}

impl Page {
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

    /// Rows are compacted into pages.
    /// Current implementation keeps it very simple.
    fn compact(&mut self, row: &Row) {
        let mut offset = 0;

        offset += self.write_int(row.id, offset);
        offset += self.write_bytes(&row.username, offset);
        offset += self.write_bytes(&row.email, offset);
    }

    /// Rows are uncompacted from pages.
    fn uncompact(&self, row: &mut Row) {
        let mut offset = 0;

        row.id = self.read_int(offset);
        offset += 4;

        row.username = self.read_bytes(offset);
        offset += USER_NAME_SIZE;

        row.email = self.read_bytes(offset);
        offset += EMAIL_SIZE;
    }
}
