use crate::{
    constants::{ROWS_PER_PAGE, ROW_SIZE},
    page::Page,
    row::Row,
};

pub struct Table {
    num_rows: usize,
    /// TODO: Better page allocations
    pages: Vec<Page>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            num_rows: 0,
            pages: vec![Page::new()],
        }
    }

    /// Insert a row into the table
    pub fn insert_row(&mut self, row: &Row) {
        let row_num = self.num_rows;
        let row_slot = self.row_slot(row_num);

        if row_slot.page_num == self.pages.len() {
            self.pages.push(Page::new());
        }

        let page = self.get_page_mut(row_slot.page_num);
        page.insert_row(row, row_slot.offset);

        self.num_rows += 1;
    }

    // Select all rows from the table
    pub fn select_rows(&self) {
        for row_num in 0..self.num_rows {
            let row_slot = self.row_slot(row_num);
            let page = self.get_page(row_slot.page_num);
            let row = page.read_row(row_slot.offset);
            println!("{}", row);
        }
    }

    pub fn stats(&self) {
        println!("Table has {} rows", self.num_rows);
        println!("Table has {} pages", self.pages.len());
    }

    pub fn get_page(&self, page_num: usize) -> &Page {
        if (self.pages.len()) < page_num {
            panic!("Tried to access page that does not exist");
        }

        &self.pages[page_num]
    }

    pub fn get_page_mut(&mut self, page_num: usize) -> &mut Page {
        if (self.pages.len()) < page_num {
            panic!("Tried to access page that does not exist");
        }

        &mut self.pages[page_num]
    }

    fn row_slot(&self, row_num: usize) -> RowSlot {
        let page_num = row_num / ROWS_PER_PAGE;

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        RowSlot {
            page_num,
            offset: byte_offset,
        }
    }
}

struct RowSlot {
    page_num: usize,
    offset: usize,
}
