use std::{rc::Rc, cell::RefCell};

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().skip(1).filter(|line| !line.is_empty());

    let root = Rc::new(Node {
        items_size:  RefCell::new(0),
        dir_size:  RefCell::new(0),
        parent: None,
        children: RefCell::new(Vec::new()),
    });

    let mut curr_dir: Rc<Node> = Rc::clone(&root);

    for line in lines {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        
        if line.starts_with("$ cd ") {
            if line_parts.last().unwrap() == &".." {
                if let Some(parent) = &curr_dir.parent {
                    curr_dir = Rc::clone(parent);
                }
            } else {
                let new_node = Rc::new(Node {
                    items_size: RefCell::new(0),
                    dir_size: RefCell::new(0),
                    parent: Some(Rc::clone(&curr_dir)),
                    children: RefCell::new(Vec::new()),
                });


                curr_dir.children.borrow_mut().push(Rc::clone(&new_node));
                curr_dir = new_node;
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            // Ignore 
        } else {
            let file_size: u32 = line_parts[0].parse::<u32>().unwrap();
            curr_dir.add_size(file_size);
        }
    }

    root.update_dir_size();
    
    let mut total_part1: u32 = 0;
    root.part1(&mut total_part1);
    
    println!("Total size of all directories under 100000: {}", total_part1);
    
    let root_size: u32 = *root.dir_size.borrow();
    let to_free: u32 = 30000000 - (70000000 - root_size);
    println!("Space to be freed: {}", to_free);
    
    let mut best: u32 = root_size;
    root.part2(&to_free,&mut best);
    println!("Smallest directory bigger than the space to be freed: {}", best);
}

struct Node {
    //name: String,
    items_size: RefCell<u32>,
    dir_size: RefCell<u32>,
    parent: Option<Rc<Node>>,
    children: RefCell<Vec<Rc<Node>>>, 
}

impl Node {
    fn add_size(& self, new_file: u32) {
        let mut items_size = self.items_size.borrow_mut();
        *items_size += new_file;
    }

    fn update_dir_size(&self) {
        if self.children.borrow().is_empty() {
            let mut dir_size = self.dir_size.borrow_mut();
             let items_size = self.items_size.borrow();
            *dir_size = *items_size;
        } 
        else {
            let mut dir_size = self.dir_size.borrow_mut();
             let items_size = self.items_size.borrow();
            *dir_size = *items_size;
            
            for child in self.children.borrow().iter() {
                child.update_dir_size(); 
                *dir_size += *child.dir_size.borrow();
            }
        }
    }

    fn part1(&self, total: &mut u32) {
        let dir_size = self.dir_size.borrow();
        if *dir_size < 100000 {
            *total += *dir_size;
        }

        for dir in self.children.borrow().iter() {
            dir.part1(total);
        }
    }

    fn part2(&self, minimum: &u32, best: &mut u32) {
        let dir_size = self.dir_size.borrow();
        if *dir_size > *minimum {
            if *dir_size < *best {
                *best = *dir_size;
            }

            for dir in self.children.borrow().iter() {
            dir.part2(minimum, best);
            }
        }
        else {
            return;
        }
    }
}

