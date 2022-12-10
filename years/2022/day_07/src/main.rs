use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::ops::Add;
use std::rc::Rc;

#[macro_use] extern crate scan_fmt;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let input_contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    // println!("DEBUG parse input: '{:#?}'",  parse(input_contents.clone()));
    println!("solution1: '{}'", solution1(parse(&input_contents.clone())));

    // TODO: should be: '3866390'
    println!("solution2: '{}'", solution2(parse(&input_contents)));

    // tree_test();

    // let tree = parse(&input_contents);
    // tree.borrow().print();
}

struct DirTree {
    name: String,
    parent: Option<Rc<RefCell<DirTree>>>,
    files: Vec<u32>,
    dirs: Vec<Rc<RefCell<DirTree>>>
}

impl DirTree {
    pub fn add_dir(&mut self, dir_tree: Rc<RefCell<DirTree>>) {
        self.dirs.push(dir_tree);
    }

    fn print(&self) {
        println!("{}", self.name);
        for dir in self.dirs.iter() {
            dir.borrow().print_with_space(2);
        }
        for file in self.files.iter() {
            println!("  {}", file);
        }
    }

    fn print_with_space(&self, space: u16) {
        let spaces = (0..space).map(|_| " ").collect::<String>();
        println!("{}{}", spaces, self.name);
        for dir in self.dirs.iter() {
            dir.borrow().print_with_space(space*2);
        }
        for file in self.files.iter() {
            println!("  {}{}", spaces, file);
        }
    }

    fn size(&self) -> u32 {
        let mut size: u32 = 0;

        size = self.files.iter().sum();

        for dir in self.dirs.iter() {
            size += dir.borrow().size();
        }

        return size;
    }
}

fn tree_test() {
    println!("tree test");

    let root = DirTree{
        name: "/".to_string(),
        parent: None,
        files: Vec::new(),
        dirs:  Vec::new(),
    };
    let root_rc = Rc::new(RefCell::new(root));

    let a = DirTree{
        name: "a".to_string(),
        parent: Some(root_rc.clone()),
        files: vec![123213,4355435,5435345],
        dirs: Vec::new(),
    };
    let a_rc = Rc::new(RefCell::new(a));

    let b = DirTree{
        name: "b".to_string(),
        parent: Some(root_rc.clone()),
        files: Vec::new(),
        dirs: Vec::new(),
    };
    let b_rc = Rc::new(RefCell::new(b));

    root_rc.borrow_mut().add_dir(a_rc);
    root_rc.borrow_mut().add_dir(b_rc.clone());

    let c = DirTree{
        name: "c".to_string(),
        parent: Some(b_rc.clone()),
        files: Vec::new(),
        dirs: Vec::new(),
    };
    let c_rc = Rc::new(RefCell::new(c));

    b_rc.borrow_mut().add_dir(c_rc);

    root_rc.borrow().print();
}

fn parse(input: &String) -> Rc<RefCell<DirTree>> {
    let root = DirTree {
        name: "/".to_string(),
        dirs: Vec::new(),
        files: Vec::new(),
        parent: None,
    };
    let root_rc = Rc::new(RefCell::new(root));

    let mut current_dir = root_rc.clone();
    for line in input.lines() {
        if line.contains("$ cd /") {
            continue;
        }

        if line.contains("$ ls") {
            continue;
        }

        if line.contains("dir") {
            let name = scan_fmt!(line, "dir {}", String).unwrap();

            let dir = DirTree {
                name: name,
                parent: Some(current_dir.clone()),
                files: Vec::new(),
                dirs: Vec::new(),
            };
            current_dir.borrow_mut().add_dir(Rc::new(RefCell::new(dir)));

            continue;
        }

        let line_has_digit = line.chars().into_iter().next().unwrap().is_numeric();
        if line_has_digit {
            let (num, _) = scan_fmt!(line, "{d} {}", u32, String).unwrap();
            current_dir.borrow_mut().files.push(num);
            continue;
        }

        if line.contains("cd ..") {
            let p = current_dir.borrow().parent.as_ref().unwrap().clone();
            current_dir = p;
            continue;
        }

        let is_cd_into_dir =  line.contains("cd") && !line.contains("..");
        if is_cd_into_dir {
            let dir_name = scan_fmt!(line, "$ cd {}", String).unwrap();

            let cd_dir = current_dir
                .borrow()
                .dirs
                .iter()
                .find(|dir| dir.borrow().name == dir_name )
                .unwrap().clone();
            current_dir = cd_dir;
            continue;
        }
    }

    return root_rc;
}

fn solution1(tree: Rc<RefCell<DirTree>>) -> u32 {
    let mut sizes: Vec<u32> = Vec::new();

    let size_root: u32 = tree.borrow().size();
    if size_root <= 100000 {
        sizes.push(size_root);
    }

    sizes.append(&mut sol1_accu(tree.borrow().dirs.clone()));

    return sizes.iter().sum();
}

fn sol1_accu(dirs: Vec<Rc<RefCell<DirTree>>>) -> Vec<u32> {
    let mut accu: Vec<u32> = Vec::new();

    for dir in dirs.iter() {
        let dir_size = dir.borrow().size();
        if dir_size <= 100000 {
            accu.push(dir_size);
        }

        let mut sub_size = sol1_accu(dir.borrow().dirs.clone());
        accu.append(&mut sub_size);
    }

    return accu;
}

fn solution2(tree: Rc<RefCell<DirTree>>) -> u32 {
    let total_available_space: u32 = 70000000;
    let needed_free_space = 30000000;

    let root_size: u32 = tree.borrow().size();

    let currently_available_space = total_available_space - root_size;
    println!("    DEBUG/solution2 - root_size: {}, currently_available_space: {}", root_size, currently_available_space);

    let mut dir_sizes: Vec<u32> = Vec::new(); 
    
    dir_sizes.push(root_size);
    dir_sizes.append(&mut sol2_accu(tree.borrow().dirs.clone()));

    dir_sizes.sort();
    println!("    DEBUG/solution2 - dir_sizes: \n{:#?}", dir_sizes);

    for dir_size in dir_sizes.iter() {
        if currently_available_space + dir_size >= needed_free_space {
            return *dir_size;
        }
    }

    return 0;
}

fn sol2_accu(dirs: Vec<Rc<RefCell<DirTree>>>) -> Vec<u32> {
    let mut accu: Vec<u32> = Vec::new();

    for dir in dirs.iter() {
        let dir_size = dir.borrow().size();
        accu.push(dir_size);

        let mut sub_size = sol1_accu(dir.borrow().dirs.clone());
        accu.append(&mut sub_size);
    }

    return accu;
}

fn solution1_old(dirs: Vec<Vec<u32>>) -> u32 {
    let out: u32 = dirs.iter()
        .map(|d| d.iter().sum())
        .filter(|s: &u32| *s <= 100000)
        .sum();

    // let out_f: Vec<u32> = dirs.iter()
    //     .map(|d| d.iter().sum())
    //     .filter(|s: &u32| *s <= 100000).collect();

    // println!("DEBUG solution1: '{:#?}'",  out_f);

    return out;
}

fn parse_old(input: &String) -> Vec<Vec<u32>> {
    let (dirs, _) = input
        .lines()
        .fold((Vec::<Vec<u32>>::new(), true), |(mut dirs, before_no_num_line), l| {
            let line_has_digit = l.chars().into_iter().next().unwrap().is_numeric();
 
            if !line_has_digit {
                return (dirs, true);
            }

            if before_no_num_line && line_has_digit {
                let (num, _) = scan_fmt!(l, "{d} {}", u32, String).unwrap();
                dirs.push(vec![num]);
                return (dirs, false)
            }


            let (num, _) = scan_fmt!(l, "{d} {}", u32, String).unwrap();
            let mut last = dirs.last().unwrap().clone();
            last.push(num);
            let leng = dirs.len();
            dirs[leng-1] = last;

            return (dirs, false);
        });

    return dirs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let data = _input_test_data_solution1();

        assert_eq!(95437, solution1(parse(&data)));
    }

    #[test]
    fn test_solution2() {
        let data = _input_test_data_solution1();

        assert_eq!(24933642, solution2(parse(&data)));
    }
}

fn _input_test_data_solution1() -> String {
return "$ cd /
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
7214296 k".to_string();
}
