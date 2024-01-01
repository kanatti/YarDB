pub struct Table {
    num_rows: u32,
    pages: Vec<Page>,
}

impl Table {
    pub fn get_page(&self, page_num: u32) -> &Page {
        if (self.pages.len() as u32) < page_num {
            panic!("Tried to access page that does not exist");
        }

        &self.pages[page_num as usize]
    }

    pub fn row_slot(&self, row_num: u32) -> RowSlot {
        let page_num = row_num / ROWS_PER_PAGE as u32;

        let row_offset = row_num % ROWS_PER_PAGE as u32;
        let byte_offset = row_offset * ROW_SIZE as u32;

        return RowSlot {
            page_num,
            offset: byte_offset,
        };
    }
}

pub struct RowSlot {
    page_num: u32,
    offset: u32,
}
