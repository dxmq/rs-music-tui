use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
}

impl<T> Page<T> {
    #[allow(unused)]
    pub fn new(items: Vec<T>, total: u32, offset: u32, limit: u32) -> Page<T> {
        Page {
            total,
            items,
            offset,
            limit,
        }
    }
}

#[derive(Clone)]
pub struct ScrollableResultPages<T> {
    #[allow(unused)]
    index: usize,
    #[allow(unused)]
    pub pages: Vec<T>,
}

impl<T> ScrollableResultPages<T> {
    #[allow(unused)]
    pub fn new() -> ScrollableResultPages<T> {
        ScrollableResultPages {
            index: 0,
            pages: vec![],
        }
    }

    #[allow(unused)]
    pub fn get_results(&self, at_index: Option<usize>) -> Option<&T> {
        self.pages.get(at_index.unwrap_or(self.index))
    }

    #[allow(unused)]
    pub fn get_mut_results(&mut self, at_index: Option<usize>) -> Option<&mut T> {
        self.pages.get_mut(at_index.unwrap_or(self.index))
    }

    #[allow(unused)]
    pub fn add_pages(&mut self, new_pages: T) {
        self.pages.push(new_pages);
        // Whenever a new page is added, set the active index to the end of the vector
        self.index = self.pages.len() - 1;
    }
}
