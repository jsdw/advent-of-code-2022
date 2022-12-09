use super::File;
use std::collections::HashMap;
use yap::{ IntoTokens, Tokens, TokenLocation };

pub fn star1(file: File) -> anyhow::Result<usize> {
    let commands = parse_input(&file.contents)?;
    let dir = Directory::from_commands(commands);

    // Add up all directories with size <= 100_000
    let sum: usize = dir
        .iter()
        .filter_map(|(_, i)| {
            match i {
                Item::Directory(dir) => Some(dir.size()),
                _ => None
            }
        })
        .filter(|s| *s <= 100000)
        .sum();

    Ok(sum)
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let commands = parse_input(&file.contents)?;
    let dir = Directory::from_commands(commands);

    let need_to_reclaim = {
        let used_space = dir.size();
        let total_space = 70000000;
        let required_space = 30000000;
        required_space - (total_space - used_space)
    };

    // Find smallest directory to delete to free up `need_to_reclaim`.
    dir
        .iter()
        .filter_map(|(_, i)| {
            match i {
                Item::Directory(dir) => Some(dir.size()),
                _ => None
            }
        })
        .filter(|s| *s >= need_to_reclaim)
        .min()
        .ok_or_else(|| anyhow::anyhow!("No suitable dirs to delete"))
}

#[derive(Debug)]
enum Item {
    Directory(Directory),
    Node(Node)
}

impl Item {
    fn unwrap_directory_mut(&mut self) -> &mut Directory {
        match self {
            Item::Directory(dir) => dir,
            _ => panic!("Not a directory")
        }
    }
}

#[derive(Debug)]
struct Directory(HashMap<String, Item>);
#[derive(Debug)]
struct Node { size: usize}

impl Directory {
    fn new() -> Directory {
        Directory(HashMap::new())
    }
    fn iter(&self) -> impl Iterator<Item = (&str, &Item)> + '_ {
        let mut stack = vec![self.0.iter()];

        std::iter::from_fn(move || {
            loop {
                let Some(mut it) = stack.pop() else {
                    return None
                };
                match it.next() {
                    Some((name, dir @ Item::Directory(inner))) => {
                        stack.push(it);
                        stack.push(inner.0.iter());
                        return Some((&**name, dir))
                    },
                    Some((name, node @ Item::Node(_))) => {
                        stack.push(it);
                        return Some((&**name, node))
                    },
                    None => {
                        // Already popped; don't replace if it's finished.
                    }
                }
            }
        })
    }
    fn size(&self) -> usize {
        self.0.values().map(|item| {
            match item {
                Item::Directory(dir) => dir.size(),
                Item::Node(Node { size }) => *size
            }
        }).sum()
    }
    fn cd_to_mut<S: AsRef<str>>(&mut self, path: &[S]) -> Option<&mut Directory> {
        let mut cwd = self;
        for piece in path {
            if let Some(Item::Directory(dir)) = cwd.0.get_mut(piece.as_ref()) {
                cwd = dir;
            } else {
                return None
            }
        }
        Some(cwd)
    }
    fn from_commands(commands: Vec<Command>) -> Directory {
        let mut root = Directory::new();
        let mut cwd = &mut root;
        let mut path = vec![];

        for cmd in commands {
            match cmd {
                Command::Cd(CdPath::Root) => {
                    path = Vec::new();
                    cwd = &mut root;
                },
                Command::Cd(CdPath::UpOne) => {
                    path.pop();
                    cwd = root.cd_to_mut(&path).unwrap();
                }
                Command::Cd(CdPath::Into(dir)) => {
                    path.push(dir.clone());
                    cwd = cwd.0
                        .entry(dir)
                        .or_insert(Item::Directory(Directory::new()))
                        .unwrap_directory_mut();
                },
                Command::Ls(output) => {
                    for item in output {
                        match item {
                            LsOutput::File { name, size } => {
                                cwd.0.entry(name).or_insert(Item::Node(Node { size }));
                            },
                            LsOutput::Dir { name } => {
                                cwd.0.entry(name).or_insert(Item::Directory(Directory::new()));
                            }
                        }
                    }
                }
            }
        }
        root
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    let mut tokens = input.into_tokens();
    tokens.many(|toks| {
        toks.skip_tokens_while(|c| c.is_whitespace());
        if toks.peek().is_none() {
            None
        } else {
            Some(parse_item(toks))
        }
    }).collect()
}

fn parse_item(cmd: &mut impl Tokens<Item=char>) -> anyhow::Result<Command> {
    if cmd.tokens("$ ".chars()) {
        if cmd.tokens("ls".chars()) {
            cmd.skip_tokens_while(|c| c.is_whitespace());
            let ls_output = parse_ls_output(cmd)?;
            Ok(Command::Ls(ls_output))
        } else if cmd.tokens("cd ".chars()) {
            if cmd.tokens("..".chars()) {
                cmd.skip_tokens_while(|c| c.is_whitespace());
                Ok(Command::Cd(CdPath::UpOne))
            } else if cmd.tokens("/".chars()) {
                cmd.skip_tokens_while(|c| c.is_whitespace());
                Ok(Command::Cd(CdPath::Root))
            } else {
                let path: String = cmd.tokens_while(|c| *c != '\n').collect();
                cmd.skip_tokens_while(|c| c.is_whitespace());
                Ok(Command::Cd(CdPath::Into(path)))
            }
        } else {
            let unrecognised: String = cmd.tokens_while(|c| *c != '\n').collect();
            anyhow::bail!("Failed to parse command '{unrecognised}'")
        }
    } else {
        let pos = cmd.location().offset();
        let unrecognised: String = cmd.tokens_while(|c| *c != '\n').collect();
        anyhow::bail!("Failed to parse from '{pos}' (starting '{unrecognised}')")
    }
}

fn parse_ls_output(input: &mut impl Tokens<Item=char>) -> anyhow::Result<Vec<LsOutput>> {
    let mut out = Vec::new();
    loop {
        if input.peek().is_none() {
            break
        } else if input.tokens("dir ".chars()) {
            let dir_name = input.tokens_while(|c| *c != '\n').collect();
            out.push(LsOutput::Dir { name: dir_name })
        } else {
            let size_str: String = input.tokens_while(|c| c.is_numeric()).collect();
            if size_str.is_empty() { break }
            let size: usize = size_str.parse().unwrap();
            input.token(' ');
            let name = input.tokens_while(|c| *c != '\n').collect();
            out.push(LsOutput::File { size, name })
        }
        input.skip_tokens_while(|c| c.is_whitespace());
    }
    Ok(out)
}

#[derive(Debug)]
enum Command {
    Ls(Vec<LsOutput>),
    Cd(CdPath),
}

#[derive(Debug)]
enum CdPath {
    Root,
    Into(String),
    UpOne
}

#[derive(Debug)]
enum LsOutput {
    File { size: usize, name: String },
    Dir { name: String }
}