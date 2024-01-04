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

        let page = &self.pager.get_page(row_slot.page_num);
        {
            let mut page = page.write().unwrap();
            page.insert_row(row, row_slot.offset);
        }

        self.num_rows += 1;
    }

    pub fn select_rows(&mut self) -> Vec<Row> {
        let mut rows = Vec::new();

        for row_num in 0..self.num_rows {
            let row_slot = self.row_slot(row_num);

            let page = &self.pager.get_page(row_slot.page_num);
            let row = {
                let page = page.read().unwrap();
                page.read_row(row_slot.offset)
            };

            rows.push(row);
        }

        rows
    }

    pub fn stats(&self) -> TableStats {
        TableStats {
            num_rows: self.num_rows,
            num_pages: self.pager.page_count(),
        }
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

pub struct TableStats {
    pub num_rows: usize,
    pub num_pages: usize,
}

struct RowSlot {
    page_num: usize,
    offset: usize,
}
