use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::sync::{Arc, Mutex};

use crate::constants::{PAGE_SIZE, TABLE_MAX_PAGES};
use crate::page::Page;

/// Holds page cache and metadata of disk files
pub struct Pager {
    file: std::fs::File,
    file_size: u64,
    pages_in_file: u64,
    pages: [Option<Arc<Mutex<Page>>>; TABLE_MAX_PAGES],
}

const INIT: Option<Arc<Mutex<Page>>> = None;

impl Pager {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(filename);

        let file = file.unwrap();

        let file_size = file.metadata().unwrap().len();

        let pages_in_file = file_size / PAGE_SIZE as u64;

        Self {
            file,
            file_size,
            pages_in_file,
            pages: [INIT; TABLE_MAX_PAGES],
        }
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn get_page(&mut self, page_num: usize) -> Arc<Mutex<Page>> {
        if page_num > TABLE_MAX_PAGES {
            panic!("Tried to access page that does not exist");
        }

        match &self.pages[page_num] {
            Some(page) => Arc::clone(page),
            None => {
                // Fetch Page from Disk
                if (page_num as u64) < self.pages_in_file {
                    let offset = page_num as u64 * 4096;

                    // TODO: Fix error handing
                    match self.file.seek(std::io::SeekFrom::Start(offset)) {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    }

                    let bytes: &mut [u8; 4096] = &mut [0; 4096];
                    match self.file.read(bytes) {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    }

                    let new_page: Arc<Mutex<Page>> = Arc::new(Mutex::new(Page::new(bytes)));

                    self.pages[page_num] = Some(Arc::clone(&new_page));

                    new_page
                } else {
                    let new_page: Arc<Mutex<Page>> = Arc::new(Mutex::new(Page::empty()));

                    self.pages[page_num] = Some(Arc::clone(&new_page));

                    new_page
                }
            }
        }
    }

    pub fn close(&mut self) {
        let file = &mut self.file;
        for (index, maybe_page) in self.pages.iter_mut().enumerate() {
            match maybe_page {
                Some(page) => Self::flush_page(file, page.clone(), index),
                None => {}
            }
        }
    }

    fn flush_page(file: &mut File, page: Arc<Mutex<Page>>, page_num: usize) {
        let offset = page_num as u64 * 4096;

        match file.seek(std::io::SeekFrom::Start(offset)) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

        let data = {
            let page = page.lock().unwrap();
            page.data.clone()
        };

        match file.write(data.as_slice()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
}
