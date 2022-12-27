use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct File {
    size: u32,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub subdirs: RefCell<Vec<Rc<Directory>>>,
    pub files: RefCell<Vec<File>>,
    pub parent: RefCell<Weak<Directory>>,
}

impl Directory {
    pub fn new(name: &str) -> Rc<Directory> {
        Rc::new(Directory {
            name: String::from(name),
            subdirs: RefCell::new(Vec::new()),
            files: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    pub fn add_file(&self, size: u32) {
        self.files.borrow_mut().push(File { size });
    }

    pub fn add_dir(self: &Rc<Self>, name: &str) {
        let dir = Directory::new(name);
        *dir.parent.borrow_mut() = Rc::downgrade(self);
        self.subdirs.borrow_mut().push(dir);
    }

    pub fn get_size(&self) -> u32 {
        let files_total: u32 = self.files.borrow().iter().map(|f| f.size).sum();
        let dirs_total: u32 = self.subdirs.borrow().iter().map(|d| d.get_size()).sum();
        files_total + dirs_total
    }

    pub fn get_total_of_dirs_below(&self, max: &u32) -> u32 {
        let dir_size = self.get_size();

        let dirs_below_total: u32 = self
            .subdirs
            .borrow()
            .iter()
            .map(|d| d.get_total_of_dirs_below(&max))
            .sum();

        if &dir_size < max {
            dir_size + dirs_below_total
        } else {
            dirs_below_total
        }
    }

    pub fn get_smallest_dir_greater_than_min(&self, min: &u32) -> Option<u32> {
        let dir_size = self.get_size();
        let dir_size_greater_than_min: Option<u32> = {
            if &dir_size > min {
                Some(dir_size)
            } else {
                None
            }
        };

        let smallest_subdir_greater_than_min: Option<u32> = self
            .subdirs
            .borrow()
            .iter()
            .filter_map(|d| d.get_smallest_dir_greater_than_min(&min))
            .min();

        let smallest_dir: Option<u32> =
            vec![dir_size_greater_than_min, smallest_subdir_greater_than_min]
                .iter()
                .filter_map(|x| *x)
                .min();

        smallest_dir
    }
}
