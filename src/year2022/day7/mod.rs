use std::{collections::HashMap, path::PathBuf, rc::Rc, cell::RefCell};
use pest::{Parser, iterators::Pair};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Parser)]
#[grammar = "year2022/day7/command.pest"]
struct CommandParser;

#[derive(Debug)]
pub enum Day7Error {}

impl Into<SolveError> for Day7Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day7Error: {:?}", self))
    }
}

#[derive(Debug)]
enum FSCommand {
    CD(String),
    LS(Vec<String>),
}

impl FSCommand {
    fn from_pair(pair: Pair<Rule>) -> FSCommand {
        match pair.as_rule() {
            Rule::cd_command => {
                let mut data_pairs = pair.into_inner();
                let cd_str = data_pairs.next().unwrap().as_str();
                FSCommand::CD(cd_str.to_string())
            },
            Rule::ls_command => {
                let ls_data: Vec<String> = pair
                    .into_inner()
                    .map(|pair| pair.as_str().to_string())
                    .collect();
                FSCommand::LS(ls_data)
            },
            _ => { panic!("unsupported command: {:?}", pair) },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum FSObjectType {
    Directory,
    File,
}

#[derive(Debug)]
struct FSObject {
    name: String,
    children: Vec<Rc<RefCell<FSObject>>>,
    _size: Option<usize>,
    fs_object_type: FSObjectType,
}

impl FSObject {
    pub fn from_str(curr_path: &str, obj_str: &str) -> Self {
        let mut line_pairs = CommandParser::parse(Rule::fs_object, obj_str).unwrap();
        let line_pair = line_pairs.next().unwrap().into_inner().next().unwrap();
        match line_pair.as_rule() {
            Rule::ls_dir => {
                let line_match = line_pair.into_inner().next().unwrap().as_str();
                let mut dir_path_buf = PathBuf::new();
                dir_path_buf.push(curr_path);
                dir_path_buf.push(line_match);
                return FSObject::new_dir(&dir_path_buf.to_str().unwrap());
            },
            Rule::ls_file => {
                let mut pairs = line_pair.into_inner();
                let ls_file_size = pairs.next().map(|size| size.as_str().parse::<usize>().unwrap()).unwrap();
                let ls_file_name = pairs.next().unwrap().as_str();
                return FSObject::new_file(ls_file_name, ls_file_size);

            },
            _ => panic!("unknown ls rule: {}", line_pair)
        }
    }

    pub fn new_dir(path: &str) -> Self {
        FSObject { name: path.to_string(), children: vec![], _size: None, fs_object_type: FSObjectType::Directory }
    }

    pub fn new_file(name: &str, size: usize) -> Self {
        FSObject { name: name.to_string(), children: vec![], _size: Some(size), fs_object_type: FSObjectType::File }
    }

    pub fn size(&self) -> usize {
        if self._size.is_some() {
            self._size.unwrap()
        } else {
            self.children.iter().fold(0, |acc_size, child| acc_size + child.borrow().size())
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    curr_dir: Box<PathBuf>,
    pub directories: HashMap<String, Rc<RefCell<FSObject>>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut directories = HashMap::new();
        let root_dir = FSObject::new_dir("/");
        directories.insert("/".to_string(), Rc::new(RefCell::new(root_dir)));
        FileSystem { curr_dir: Box::new(PathBuf::new()), directories }
    }

    pub fn execute(&mut self, command: FSCommand) {
        match command {
            FSCommand::CD(path) => {
                if &path == ".." {
                    self.curr_dir.pop();
                }  else {
                    self.curr_dir.push(path);
                }
            },
            FSCommand::LS(list) => {
                let curr_path = self.curr_dir.to_str().unwrap();
                let fs_children = list.iter().fold(vec![], |mut children, line| {
                    let child_fs_object = FSObject::from_str(curr_path, line);
                    children.push(child_fs_object);

                    children
                });

                let directory_keys: Vec<String> = self.directories.keys().map(|key| key.clone()).collect();
                
                for child in fs_children {
                    let child_name = child.name.clone();
                    let child_type = child.fs_object_type.clone();

                    let rc_child = Rc::new(RefCell::new(child));
                    if child_type == FSObjectType::Directory && !directory_keys.contains(&child_name) {
                        self.directories.insert(child_name, Rc::clone(&rc_child));
                    }
                    let fs_object = self.directories.get_mut(curr_path).unwrap();
                    fs_object.borrow_mut().children.push(Rc::clone(&rc_child));
                }
            },
        };
    }
}

pub struct Day7Handler {}
impl<'a> Day7Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day7Handler {}) }
    pub fn solve_1(&self, input: &str) -> Result<String, Day7Error> {
        let mut commands = CommandParser::parse(Rule::command_file, input.clone()).unwrap();
        let command_file = commands.next().unwrap();
        let mut command_list: Vec<FSCommand> = vec![];
        for command_line in command_file.into_inner() {
            let command_info_opt = command_line.into_inner().next();
            if let Some(command_info) = command_info_opt {
                command_list.push(FSCommand::from_pair(command_info));
            }
        }

        let mut file_system = FileSystem::new();

        for command in command_list {
            file_system.execute(command);
        }
        
        let mut total = 0;
        for directory in file_system.directories {
            let dir_size = directory.1.borrow().size();
            if dir_size <= 100000 {
                total = total + dir_size;
            }
        }

        Ok(total.to_string())
    }
    
    pub fn solve_2(&self, input: &str) -> Result<String, Day7Error> {
        let mut commands = CommandParser::parse(Rule::command_file, input.clone()).unwrap();
        let command_file = commands.next().unwrap();
        let mut command_list: Vec<FSCommand> = vec![];
        for command_line in command_file.into_inner() {
            let command_info_opt = command_line.into_inner().next();
            if let Some(command_info) = command_info_opt {
                command_list.push(FSCommand::from_pair(command_info));
            }
        }

        let mut file_system = FileSystem::new();

        for command in command_list {
            file_system.execute(command);
        }
        
        let available_disk: usize = 70000000;
        let used = file_system.directories.get("/").unwrap().borrow().size();
        let unused = available_disk - used;
        let needed = 30000000;
        let mut min_delete = available_disk;
        for directory in file_system.directories {
            let dir_size = directory.1.borrow().size();

            if unused + dir_size >= needed && min_delete > dir_size {
                min_delete = dir_size;
            }
        }

        Ok(min_delete.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day7Handler {
    fn get_day(&self) -> String { String::from("7") }
    fn solve(&self, problem: &str, input: &str) -> Result<String, SolveError> {
        let result = if problem == "1" {
            self.solve_1(input)
        } else {
            self.solve_2(input)
        };

        result.map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::AdventSolution;
    use super::Day7Handler;

    fn get_input<'a>() -> &'a str {
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
    }

    #[test]
    fn get_day() {
        let handler = Day7Handler::new();
        assert!(&handler.get_day() == "7");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day7Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("95437"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("24933642"));
    }
}
