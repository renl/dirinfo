use walkdir::{DirEntry, WalkDir};

pub enum BlockSize {
    Kb100,
    Kb500,
    Mb(usize),
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
    pub fn new() -> DirInfo {
        DirInfo {
            all: None,
            errors: None,
            directories: None,
            files: None,
            symlinks: None,
        }
    }

    pub fn pull(self, root_dir: &str) -> DirInfo {
        self.all(root_dir)
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

    pub fn get_file_size_distribution(&self, blocksize: BlockSize) -> Vec<usize> {
        let blk: usize = match blocksize {
            BlockSize::Kb100 => 100_000usize,
            BlockSize::Kb500 => 500_000usize,
            BlockSize::Mb(x) => x * 1000_000usize,
        };
        let biggest = if let Some(ref files) = self.files {
            files.into_iter().fold(0, |max, d| {
                if d.metadata().unwrap().len() > max {
                    d.metadata().unwrap().len()
                } else {
                    max
                }
            })
        } else {
            0
        };
        let mut distribution: Vec<usize> = vec![0; (biggest as usize / blk) + 1];
        if let Some(ref files) = self.files {
            files.into_iter().for_each(|f| {
                distribution[f.metadata().unwrap().len() as usize / blk as usize] += 1
            });
        }
        distribution
    }

    pub fn get_files_size(&self) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .fold(0, |acc, s| acc + s.metadata().unwrap().len() as usize),
            _ => 0,
        }
    }

    pub fn get_files_size_by_file_ext(&self, ext: &str) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().ends_with(ext))
                .fold(0, |acc, f| acc + f.metadata().unwrap().len() as usize),
            _ => 0,
        }
    }

    pub fn get_num_files_by_file_ext(&self, ext: &str) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().ends_with(ext))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn get_hidden_files_size(&self) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, f| acc + f.metadata().unwrap().len() as usize),
            _ => 0,
        }
    }

    pub fn get_num_files(&self) -> usize {
        match self.files {
            Some(ref files) => files.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn get_num_hidden_files(&self) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn get_num_directories(&self) -> usize {
        match self.directories {
            Some(ref directories) => directories.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn get_num_hidden_directories(&self) -> usize {
        match self.directories {
            Some(ref directories) => directories
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn get_num_symlinks(&self) -> usize {
        match self.symlinks {
            Some(ref symlinks) => symlinks.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    pub fn get_deepest_depth(&self) -> usize {
        if let Some(ref files) = self.files {
            files
                .iter()
                .fold(0, |max, d| if d.depth() > max { d.depth() } else { max })
        } else {
            0
        }
    }

    pub fn get_num_files_by_depth(&self) {}

    pub fn get_num_directories_by_depth(&self) {}

    pub fn get_num_symlinks_by_depth(&self) {}

    pub fn get_files_size_by_depth(&self) {}

    pub fn get_hidden_files_size_by_depth(&self) {}

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
        println!(
            "{:#?}",
            DirInfo::new()
                .pull(".")
                .get_file_size_distribution(BlockSize::Kb100)
        );
    }

    #[test]
    fn splitfiles() {
        let d = DirInfo::new().pull("/etc");
        println!("{:#?} ", d);
    }

    #[test]
    fn byabsolutepath() {
        println!(
            "{:#?}",
            DirInfo::new()
                .pull(std::env::current_dir().unwrap().to_str().unwrap())
                .files
        );
    }

    #[test]
    fn hiddenfilesize() {
        println!("{}", DirInfo::new().pull("../..").get_hidden_files_size());
    }

    #[test]
    fn hiddenfilenum() {
        println!("{}", DirInfo::new().pull("../..").get_num_hidden_files());
    }

    #[test]
    fn filesizebyext() {
        println!(
            "{}",
            DirInfo::new()
                .pull("/etc")
                .get_files_size_by_file_ext(".conf")
        );
    }

    #[test]
    fn dirinfonew() {
        println!("{:#?}", DirInfo::new().pull("../.."));
    }

    #[test]
    fn filesize() {
        println!("{}", DirInfo::new().pull("../..").get_files_size());
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
