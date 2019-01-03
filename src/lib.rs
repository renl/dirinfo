use walkdir::{DirEntry, WalkDir};

pub enum BlockSize {
    Kb100,
    Kb500,
    Mb(u64),
}

#[derive(Debug)]
pub struct Error {
    error: Option<std::io::Error>,
    path: Option<String>,
    depth: usize,
}

impl Error {
    pub fn new(path: Option<String>, depth: usize, error: Option<std::io::Error>) -> Error {
        Error {
            path: path,
            depth: depth,
            error: error,
        }
    }

    fn from(e: walkdir::Error) -> Error {
        let kind = match &e.io_error() {
            Some(ref err) => err.kind(),
            _ => std::io::ErrorKind::Other,
        };
        let d = e.depth();
        let s = String::from(e.path().unwrap().to_str().unwrap());
        let new_e = std::io::Error::new(kind, e);
        Error::new(Some(s), d, Some(new_e))
    }
}

#[derive(Debug)]
pub struct DirInfo {
    all: Option<Vec<DirEntry>>,
    errors: Option<Vec<Error>>,
    directories: Option<Vec<DirEntry>>,
    files: Option<Vec<DirEntry>>,
    symlinks: Option<Vec<DirEntry>>,
}

impl DirInfo {
    pub fn new(root: &str) -> DirInfo {
        DirInfo {
            all: None,
            errors: None,
            directories: None,
            files: None,
            symlinks: None,
        }
        .all(root)
        .all_directories()
        .all_files()
        .all_symlinks()
    }

    fn all(mut self, root: &str) -> DirInfo {
        let mut direntries: Vec<DirEntry> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();
        WalkDir::new(root).into_iter().for_each(|de| {
            // println!("{:#?}", de);
            match de {
                Ok(d) => direntries.push(d),
                Err(e) => errors.push(Error::from(e)),
            }
        });
        self.all = Some(direntries);
        self.errors = Some(errors);
        self
    }

    pub fn file_size_distribution(&self, blocksize: BlockSize) -> Vec<u64> {
        let blk: u64 = match blocksize {
            BlockSize::Kb100 => 100_000u64,
            BlockSize::Kb500 => 500_000u64,
            BlockSize::Mb(x) => x * 1000_000u64,
        };
        let biggest = if let Some(ref files) = self.files {
            files.into_iter().fold(0, |max, d| {
                if d.metadata().unwrap().len() > max {
                    println!("{}", d.file_name().to_str().unwrap());
                    d.metadata().unwrap().len()
                } else {
                    max
                }
            })
        } else {
            0
        };
        println!("Vec length is {}", (biggest / blk) as usize);
        let mut distribution: Vec<u64> = vec![0; (biggest / blk) as usize + 1];
        if let Some(ref files) = self.files {
            files.into_iter().for_each(|f| {
                distribution[f.metadata().unwrap().len() as usize / blk as usize] += 1
            });
        }
        distribution
    }

    pub fn total_files_size(&self) -> u64 {
        match self.files {
            Some(ref files) => files
                .iter()
                .fold(0, |acc, s| acc + s.metadata().unwrap().len()),
            _ => 0,
        }
    }

    pub fn total_files_size_by_file_ext(&self, ext: &str) -> u64 {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().ends_with(ext))
                .fold(0, |acc, f| acc + f.metadata().unwrap().len()),
            _ => 0,
        }
    }

    pub fn total_num_files_by_file_ext(&self, ext: &str) -> u64 {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().ends_with(ext))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn total_hidden_files_size(&self) -> u64 {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, f| acc + f.metadata().unwrap().len()),
            _ => 0,
        }
    }

    pub fn total_num_files(&self) -> u64 {
        match self.files {
            Some(ref files) => files.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn total_num_hidden_files(&self) -> u64 {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn total_num_directories(&self) -> u64 {
        match self.directories {
            Some(ref directories) => directories.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn total_num_hidden_directories(&self) -> u64 {
        match self.directories {
            Some(ref directories) => directories
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn total_num_symlinks(&self) -> u64 {
        match self.symlinks {
            Some(ref symlinks) => symlinks.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    fn all_directories(mut self) -> DirInfo {
        let mut entries: Vec<DirEntry> = Vec::new();
        if let Some(ref all) = self.all {
            for entry in all {
                if entry.file_type().is_dir() {
                    entries.push(entry.clone());
                }
            }
            self.directories = Some(entries);
        };
        self
    }

    fn all_files(mut self) -> DirInfo {
        let mut entries: Vec<DirEntry> = Vec::new();
        if let Some(ref all) = self.all {
            for entry in all {
                if entry.file_type().is_file() {
                    entries.push(entry.clone());
                }
            }
            self.files = Some(entries);
        }
        self
    }

    fn all_symlinks(mut self) -> DirInfo {
        let mut entries: Vec<DirEntry> = Vec::new();
        if let Some(ref all) = self.all {
            for entry in all {
                if entry.file_type().is_symlink() {
                    entries.push(entry.clone());
                }
            }
            self.symlinks = Some(entries);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribution() {
        println!("{:#?}", DirInfo::new(".").file_size_distribution(BlockSize::Kb100));
    }

    #[test]
    fn splitfiles() {
        let d = DirInfo::new("/etc");
        println!("{:#?} ", d);
    }

    #[test]
    fn byabsolutepath() {
        println!(
            "{:#?}",
            DirInfo::new(std::env::current_dir().unwrap().to_str().unwrap()).files
        );
    }

    #[test]
    fn hiddenfilesize() {
        println!("{}", DirInfo::new("../..").total_hidden_files_size());
    }

    #[test]
    fn hiddenfilenum() {
        println!("{}", DirInfo::new("../..").total_num_hidden_files());
    }

    #[test]
    fn filesizebyext() {
        println!(
            "{}",
            DirInfo::new("../..").total_files_size_by_file_ext(".toml")
        );
    }

    #[test]
    fn dirinfonew() {
        println!("{:#?}", DirInfo::new("../.."));
    }

    #[test]
    fn filesize() {
        println!("{}", DirInfo::new("../..").total_files_size());
    }

    #[test]
    fn scratchpad() {
        let a = Some(Some(String::from("hello")));
        let mut z = Vec::<String>::new();
        if let Some(ref b) = a {
            if let Some(c) = b {
                z.push(c.to_string());
            }
        }
        println!("{:?}", a);

        let i = 32874;
        let j = i as f32 / 100f32;
        let k: u32 = i / 100;

        println!("{} {}", j, k);
    }
}
