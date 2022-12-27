#[derive(Debug)]
pub enum ListDirectoryOutput {
    Dir(String),
    File(u32, String),
}

#[derive(Debug)]
pub enum Command {
    GoUpToTopLevel,
    GoUpLevel,
    GoInTo(String),
    ListDirectory(Vec<ListDirectoryOutput>),
}

pub fn from_io(input: impl Iterator<Item = String>) -> Option<Vec<Command>> {
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
