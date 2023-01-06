use std::{collections::HashMap, error::Error};

use pest::{Parser, iterators::Pair};


#[derive(Parser)]
#[grammar = "day7.pest"]
pub struct FsCmdParser;

#[derive(Debug)]
pub enum LsCmdOutput {
    DirOutput(String),
    FileOutput(String, usize),
}
#[derive(Debug)]
pub enum FsCmd {
    CdParent,
    CdRoot,
    Cd(String),
    Ls(Vec<LsCmdOutput>),
}

fn parse_cmds(e: Pair<Rule>) -> Result<FsCmd, &str> {
    match e.as_rule() {
        Rule::lsCmd => {            
            let output_lines: Result<Vec<LsCmdOutput>, &str> = e
            .into_inner()
            .map(|l| {
                match l.as_rule() {
                    Rule::dirOutput => 
                    Ok(LsCmdOutput::DirOutput(l.as_str().to_string())),
                    Rule::fileOutput => { 
                        let mut inner = l.into_inner();
                        let file_size =    inner.next().ok_or("lsCmd fileOutput is missing file size")?.as_str();
                        let file_name =    inner.next().ok_or("lsCmd fileOutput is missing file name")?.as_str();
                        let file_size = file_size.parse::<usize>().map_err(|_|"failed to parse file_size")?;
                        Ok(LsCmdOutput::FileOutput(file_name.to_string(), file_size))
                },
                _ => Err("invalid syntax for lsCmd output - expected dirOutput | fileOutput"),
            }
        })
            .collect();
            Ok(FsCmd::Ls(output_lines?))
        }
        Rule::cdCmd => { 
            let p =    e.into_inner().next().ok_or("Failed to find CD path")?;
            match p.as_rule() {
                Rule::cdRoot => Ok(FsCmd::CdRoot),
                Rule::cdParent => Ok(FsCmd::CdParent),
                Rule::cdPath => {
                    Ok(FsCmd::Cd(p.as_str().to_string()))
                },
                _ => Err("Invalid syntax, CdCmd can only contain cdRoot, cdParent or cdPath")
            }
           
        }
        _ => Err("Invalid top level cmd, only lsCmd or cdCmd are currently supported")
    }
}

struct ElfFs {
    cwd: Vec<String>,
    file_sizes: HashMap<Vec<String>, usize>,
}

impl ElfFs {
    fn empty() -> ElfFs {
        ElfFs { 
            cwd: vec!["/".to_string()],
            file_sizes: HashMap::new(),
          }
    }

    fn fold_cmd<'a>(acc: &'a mut ElfFs, cmd: &FsCmd) -> &'a mut ElfFs {
        match cmd {
            FsCmd::CdParent => acc.cd_parent(),
            FsCmd::CdRoot => acc.cd_root(),
            FsCmd::Cd(path) => acc.cd(path.clone()),
            FsCmd::Ls(contents) => acc.populate_files(contents),
        }
    }

    fn dir_sizes(&self) -> HashMap<String, usize> {
        let mut dirs = HashMap::new();

        for (path, size) in self.file_sizes.iter() {
            let path_parts = &path[0..path.len()-1];
            let absolute_paths = path_parts.iter().fold(vec![], |acc, p| {
                if acc.is_empty() {
                    vec![p.clone()]
                } else {
                    let parent =    acc.last().unwrap();
                    let path = format!("{}{}/", parent, p);
                    acc.into_iter().chain(vec![path].into_iter()).collect()
                }
            });
            for dir in absolute_paths {
                (*dirs.entry(dir.clone()).or_insert(0)) += size;
            }
        }
        dirs
    }

    fn populate_files(&mut self, cs: &Vec<LsCmdOutput>) -> &mut ElfFs {
        for o in cs {
            match o {
                LsCmdOutput::DirOutput(_) => (),
                LsCmdOutput::FileOutput(n, s) => {
                    let mut path = self.cwd.clone();
                    path.push(n.to_string());
                    self.file_sizes.insert(path, *s);
                } 
            }
        }
        self
    }
    
    fn cd_root(&mut self) -> &mut ElfFs {
        self.cwd = vec!["/".to_string()];
        self
    }

    fn cd_parent(&mut self) -> &mut ElfFs {
        self.cwd.pop();
        self
    }

    fn cd(&mut self, path: String) -> &mut ElfFs {
        self.cwd.push(path);
        self
    }
}

pub fn result(inputs: String) -> Result<(), Box<dyn Error>> {
    let parsed = FsCmdParser::parse(Rule::fsCmd, &inputs)?;

    let cmds: Result<Vec<FsCmd>, &str> = parsed.map(parse_cmds).collect();
    let cmds = cmds?;
    let mut init = ElfFs::empty();
    let fs =    cmds.iter().fold(&mut init, ElfFs::fold_cmd);
    let dir_sizes = fs.dir_sizes();

    let result: usize = dir_sizes.iter()
        .filter(|(_, &size)| size <= 100000)
        .map(|(_, &size)| size)
        .sum();
    
    println!("Result: {}", result);
    let total_fs_size: usize = 70000000;
    let required_free_space: usize = 30000000;
    let root_dir_size = dir_sizes.get("/").ok_or("Cannot find / (root) in dir_sizes")?;
    let free_space = total_fs_size - root_dir_size;
    let space_needed_to_free = required_free_space - free_space;
    println!("Space needed to free: {}", space_needed_to_free);
    let mut big_enough_dirs:Vec<usize> =    dir_sizes.iter()
        .filter(|(_, &size)| size >= space_needed_to_free)
        .map(|(_, &s)|  s)
        .collect();
    big_enough_dirs.sort();
    let res_s = big_enough_dirs.first().ok_or("empty big_enough_dirs vec, no dirs are big enough!")?;
    println!("Part2 result:  {}", res_s);
        
    Ok(())
}
