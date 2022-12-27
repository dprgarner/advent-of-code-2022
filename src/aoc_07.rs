use std::rc::Rc;

mod commands;
mod directories;

use commands::{Command, ListDirectoryOutput};
use directories::Directory;

fn build_file_system(commands: &Vec<Command>) -> Rc<Directory> {
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
                                current.add_dir(name);
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
    let commands = commands::from_io(input).expect("Could not parse commands");
    let dir = build_file_system(&commands);

    let cut_off = 100000;
    Ok(dir.get_total_of_dirs_below(&cut_off))
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<u32, &'static str> {
    let commands = commands::from_io(input).expect("Could not parse commands");
    let dir = build_file_system(&commands);

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
