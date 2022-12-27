use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum ListDirectoryOutput {
    Dir(String),
    File(u32, String),
}

#[derive(Debug)]
enum Command {
    GoUpToTopLevel,
    GoUpLevel,
    GoInTo(String),
    ListDirectory(Vec<ListDirectoryOutput>),
}

fn parse_io_log(input: impl Iterator<Item = String>) -> Option<Vec<Command>> {
    let mut ls_output = Vec::new();
    let mut commands = Vec::new();

    for line in input {
        if line.starts_with("$") {
            if ls_output.len() > 0 {
                commands.push(Command::ListDirectory(ls_output));
            }
            ls_output = Vec::new();

            if line == "$ cd /" {
                commands.push(Command::GoUpToTopLevel);
            } else if line == "$ cd .." {
                commands.push(Command::GoUpLevel);
            } else if line.starts_with("$ cd ") {
                commands.push(Command::GoInTo(line.split("$ cd ").nth(1)?.to_string()));
            }
        } else {
            if line.starts_with("dir ") {
                ls_output.push(ListDirectoryOutput::Dir(
                    line.split(" ").nth(1)?.to_string(),
                ));
            } else {
                let file_size: u32 = line.split(" ").nth(0)?.parse().ok()?;
                let file_name = line.split(" ").nth(1)?.to_string();
                ls_output.push(ListDirectoryOutput::File(file_size, file_name));
            }
        }
    }
    if ls_output.len() > 0 {
        commands.push(Command::ListDirectory(ls_output));
    }

    Some(commands)
}

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    subdirs: RefCell<Vec<Rc<Directory>>>,
    files: RefCell<Vec<File>>,
    parent: RefCell<Weak<Directory>>,
}

fn add_new_dir(parent: &Rc<Directory>, name: &str) {
    let dir = Directory::new(name);
    *dir.parent.borrow_mut() = Rc::downgrade(parent);
    parent.subdirs.borrow_mut().push(dir);
}

impl Directory {
    fn new(name: &str) -> Rc<Directory> {
        Rc::new(Directory {
            name: String::from(name),
            subdirs: RefCell::new(Vec::new()),
            files: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    fn add_file(&self, size: u32) {
        self.files.borrow_mut().push(File { size });
    }

    fn get_size(&self) -> u32 {
        let files_total: u32 = self.files.borrow().iter().map(|f| f.size).sum();
        let dirs_total: u32 = self.subdirs.borrow().iter().map(|d| d.get_size()).sum();
        files_total + dirs_total
    }

    fn get_total_of_dirs_below(&self, max: &u32) -> u32 {
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

    fn get_smallest_dir_greater_than_min(&self, min: &u32) -> Option<u32> {
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

fn parse_commands(commands: &Vec<Command>) -> Rc<Directory> {
    let root = Directory::new("/");
    let mut current = Rc::clone(&root);

    for command in commands {
        match command {
            Command::GoUpToTopLevel => {
                current = Rc::clone(&root);
            }
            Command::GoUpLevel => {
                let parent = current
                    .parent
                    .borrow_mut()
                    .upgrade()
                    .expect("Could not find directory above");
                current = Rc::clone(&parent);
            }
            Command::GoInTo(name) => {
                let option_target;
                {
                    let options = current.subdirs.borrow();
                    option_target = Rc::clone(
                        options
                            .iter()
                            .find(|x| &x.name == name)
                            .expect("Could not find directory with this name"),
                    );
                }
                current = option_target
            }
            Command::ListDirectory(files_or_dirs) => {
                if current.files.borrow().len() + current.subdirs.borrow().len() == 0 {
                    for file_or_dir in files_or_dirs {
                        match file_or_dir {
                            ListDirectoryOutput::Dir(name) => {
                                add_new_dir(&current, name);
                            }
                            ListDirectoryOutput::File(size, _) => {
                                current.add_file(size.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    root
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<u32, &'static str> {
    let commands = parse_io_log(input).expect("Could not parse commands");
    let dir = parse_commands(&commands);

    Ok(dir.get_total_of_dirs_below(&100000))
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<u32, &'static str> {
    let commands = parse_io_log(input).expect("Could not parse commands");
    let dir = parse_commands(&commands);

    let update_space_required: u32 = 30000000;
    let total_file_system_size: u32 = 70000000;
    let used_space: u32 = dir.get_size();
    assert!(used_space < total_file_system_size);

    let free_space: u32 = total_file_system_size - used_space;
    assert!(update_space_required > free_space);

    let required_space = update_space_required - free_space;

    dir.get_smallest_dir_greater_than_min(&required_space)
        .ok_or("Could not find a directory over this size")
}
