pub const INPUT: &str = "$ cd /
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
7214296 k";

use std::cell::RefCell;
use std::rc::Rc;

struct Path {
    name: String,
    parent: Option<Rc<RefCell<Path>>>,
    kind: PathKind,
}

enum PathKind {
    Dir(Vec<Rc<RefCell<Path>>>),
    File(usize),
}

impl Path {
    fn recurse(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let description = match self.kind {
            PathKind::Dir(_) => "(dir)".to_string(),
            PathKind::File(size) => format!("(file, size={})", size),
        };
        writeln!(
            f,
            "{}- {} {}",
            vec![" "; depth * 2].join(""),
            self.name,
            description
        )?;
        if let PathKind::Dir(ls) = &self.kind {
            ls.iter()
                .map(|file| file.borrow().recurse(f, depth + 1))
                .collect::<Result<_, _>>()?;
        };
        Ok(())
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.recurse(f, 0)
    }
}

fn parse(input: &str) -> Rc<RefCell<Path>> {
    let mut lines = input.lines().peekable();
    let mut root: Option<Rc<RefCell<Path>>> = None;
    let mut node: Option<Rc<RefCell<Path>>> = None;
    while let Some(line) = lines.next() {
        match &line[2..4] {
            "cd" => {
                let dir = &line[5..];
                node = match dir {
                    "/" => {
                        root = Some(Rc::new(RefCell::new(Path {
                            name: dir.to_string(),
                            parent: None,
                            kind: PathKind::Dir(vec![]),
                        })));
                        root.clone()
                    }
                    ".." => node.as_ref().and_then(|n| n.borrow().parent.clone()),
                    _ => node.as_ref().and_then(|n| match &n.borrow().kind {
                        PathKind::Dir(dirs) => dirs
                            .iter()
                            .find(|n| n.borrow().name == dir)
                            .map(|n| n.clone()),
                        _ => node.clone(),
                    }),
                };
            }
            "ls" => {
                let Some(node) = node.as_mut() else {
                    continue;
                };
                let PathKind::Dir(ls) = &mut node.borrow_mut().kind else {
                    continue;
                };
                while lines
                    .peek()
                    .map(|l| l.as_bytes()[0] != b'$')
                    .unwrap_or(false)
                {
                    match lines.next().and_then(|line| line.trim().split_once(" ")) {
                        Some(("dir", name)) => ls.push(Rc::new(RefCell::new(Path {
                            name: name.to_string(),
                            parent: Some(node.clone()),
                            kind: PathKind::Dir(vec![]),
                        }))),
                        Some((size, name)) => ls.push(Rc::new(RefCell::new(Path {
                            name: name.to_string(),
                            parent: Some(node.clone()),
                            kind: PathKind::File(size.parse().unwrap()),
                        }))),
                        None => (),
                    }
                }
            }
            _ => (),
        }
    }
    root.unwrap()
}

pub fn part_one(input: &str) -> usize {
    let tree = parse(input);
    println!("{}", tree.borrow());
    let mut total = 0;
    let mut sum_smallest = |size: usize| {
        if size <= 100000 {
            total += size;
        }
    };
    size_search(&tree, &mut sum_smallest);
    total
}

pub fn part_two(input: &str) -> usize {
    let tree = parse(input);
    let target = 30000000 + size_search(&tree, &mut |_| ()) - 70000000;
    let mut best = usize::MAX;
    let mut find_smallest = |size: usize| {
        if size >= target {
            best = best.min(size);
        }
    };
    size_search(&tree, &mut find_smallest);
    best
}

fn size_search(node: &Rc<RefCell<Path>>, f: &mut impl FnMut(usize)) -> usize {
    match &node.borrow().kind {
        PathKind::File(size) => *size,
        PathKind::Dir(ls) => {
            let size: usize = ls.iter().map(|file| size_search(file, f)).sum();
            f(size);
            size
        }
    }
}
