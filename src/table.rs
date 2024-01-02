use std::fs;

use crate::{
    constants::{ROWS_PER_PAGE, ROW_SIZE},
    pager::Pager,
    row::Row,
};

pub struct Table {
    num_rows: usize,
    pager: Pager,
    name: String,
}

impl Table {
    pub fn new(name: String) -> Self {
        let pager = Pager::new(&format!("{}.db", name));

        let num_rows = fs::read_to_string(format!("{}.meta", name))
            .unwrap_or(String::from("0"))
            .parse()
            .unwrap();

        Table {
            pager,
            num_rows,
            name,
        }
    }

    /// Insert a row into the table
    pub fn insert_row(&mut self, row: &Row) {
        let row_num = self.num_rows;
        let row_slot = self.row_slot(row_num);

        let page = self.pager.get_page(row_slot.page_num);
        page.borrow_mut().insert_row(row, row_slot.offset);

        self.num_rows += 1;
    }

    // Select all rows from the table
    pub fn select_rows(&mut self) {
        for row_num in 0..self.num_rows {
            let row_slot = self.row_slot(row_num);
            let page = self.pager.get_page(row_slot.page_num);
            let row = page.borrow().read_row(row_slot.offset);
            println!("{}", row);
        }
    }

    pub fn stats(&self) {
        println!("Table has {} rows", self.num_rows);
        println!("Table has {} pages", self.pager.page_count());
    }

    pub fn close(&mut self) {
        fs::write(format!("{}.meta", self.name), self.num_rows.to_string()).unwrap();

        self.pager.close();
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
