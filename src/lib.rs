use walkdir::{DirEntry, WalkDir};
#[cfg(test)]
mod tests;

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
    /// Create a new empty DirInfo struct

    pub fn new() -> DirInfo {
        DirInfo {
            all: None,
            errors: None,
            directories: None,
            files: None,
            symlinks: None,
        }
    }

    /// Populate DirInfo fields with directory information pulled with root_dir arg
    /// directory specifying the root directory to pull from

    pub fn pull(self, root_dir: &str) -> DirInfo {
        self.all(root_dir)
            .all_directories()
            .all_files()
            .all_symlinks()
    }

    fn all(mut self, root: &str) -> DirInfo {
        let mut direntries: Vec<DirEntry> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();
        WalkDir::new(root).into_iter().for_each(|de| match de {
            Ok(d) => direntries.push(d),
            Err(e) => errors.push(Error::from(e)),
        });
        self.all = Some(direntries);
        self.errors = Some(errors);
        self
    }

    /// For all files found in the directory hierarchy, create a histogram of file
    /// sizes with the bin size of histogram specified by blocksize arg

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

    /// Calculate the total file size in bytes for all the files found in directory
    /// hierarchy

    pub fn get_files_size(&self) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .fold(0, |acc, s| acc + s.metadata().unwrap().len() as usize),
            _ => 0,
        }
    }

    /// Calculate the total file size in bytes for all files with file extension
    /// of ext arg found in directory hierarchy

    pub fn get_files_size_by_file_ext(&self, ext: &str) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().ends_with(ext))
                .fold(0, |acc, f| acc + f.metadata().unwrap().len() as usize),
            _ => 0,
        }
    }

    /// Calculate the total number of files with the extension specified by ext arg
    /// found in directory hierarchy

    pub fn get_num_files_by_file_ext(&self, ext: &str) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().ends_with(ext))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    /// Calculate the total file size in bytes for all hidden files found in directory
    /// hierarchy

    pub fn get_hidden_files_size(&self) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, f| acc + f.metadata().unwrap().len() as usize),
            _ => 0,
        }
    }

    /// Calculate the total number of files found in directory hierarchy

    pub fn get_num_files(&self) -> usize {
        match self.files {
            Some(ref files) => files.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    /// Calculate the total number of hidden files found in directory hierarchy

    pub fn get_num_hidden_files(&self) -> usize {
        match self.files {
            Some(ref files) => files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    /// Calculate the total number of sub directories found in directory hierarchy

    pub fn get_num_directories(&self) -> usize {
        match self.directories {
            Some(ref directories) => directories.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    /// Calculate the total number of hidden sub directories found in the directory
    /// hierarchy

    pub fn get_num_hidden_directories(&self) -> usize {
        match self.directories {
            Some(ref directories) => directories
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    /// Calculate the total number of symbolic links found in the directory hierarchy

    pub fn get_num_symlinks(&self) -> usize {
        match self.symlinks {
            Some(ref symlinks) => symlinks.iter().fold(0, |acc, _f| acc + 1),
            _ => 0,
        }
    }

    fn deepest_depth(files: &Vec<DirEntry>) -> usize {
        files
            .iter()
            .fold(0, |max, d| if d.depth() > max { d.depth() } else { max })
    }

    /// Identify maximum depth of directory hierarchy

    pub fn get_deepest_depth(&self) -> usize {
        if let Some(ref files) = self.files {
            Self::deepest_depth(files)
        } else {
            0
        }
    }

    fn entry_depth_distri(entries: &Vec<DirEntry>) -> Vec<u32> {
        let deepest = Self::deepest_depth(entries);
        let mut depth_distri = vec![0u32; deepest];
        entries.iter().for_each(|f| {
            if f.depth() > 0 {
                depth_distri[f.depth() - 1] += 1
            }
        });
        depth_distri
    }

    /// Calculate distribution of number of files by depth level in directory hierarchy

    pub fn get_num_files_by_depth(&self) -> Vec<u32> {
        if let Some(ref files) = self.files {
            Self::entry_depth_distri(files)
        } else {
            vec![0]
        }
    }

    /// Calculate distribution of number of sub directories by depth level in directory
    /// hierarchy

    pub fn get_num_directories_by_depth(&self) -> Vec<u32> {
        if let Some(ref directories) = self.directories {
            Self::entry_depth_distri(directories)
        } else {
            vec![0]
        }
    }

    /// Calculate distribution of number of symbolic links by depth level in directory
    /// hierarchy

    pub fn get_num_symlinks_by_depth(&self) -> Vec<u32> {
        if let Some(ref symlinks) = self.symlinks {
            Self::entry_depth_distri(symlinks)
        } else {
            vec![0]
        }
    }

    /// Calculate distribution of file size by depth level in directory hierarchy

    pub fn get_files_size_by_depth(&self) -> Vec<usize> {
        if let Some(ref files) = self.files {
            let deepest = Self::deepest_depth(files);
            files.iter().fold(vec![0usize; deepest], |mut acc, f| {
                acc[f.depth() - 1] += f.metadata().unwrap().len() as usize;
                acc
            })
        } else {
            vec![0]
        }
    }

    /// Calculate distribution of hidden file size by depth level in directory hierarchy

    pub fn get_hidden_files_size_by_depth(&self) -> Vec<usize> {
        if let Some(ref files) = self.files {
            let deepest = Self::deepest_depth(files);
            files.iter().fold(vec![0usize; deepest], |mut acc, f| {
                if f.file_name().to_str().unwrap().starts_with(".") {
                    acc[f.depth() - 1] += f.metadata().unwrap().len() as usize;
                }
                acc
            })
        } else {
            vec![0]
        }
    }

    /// Calculate distribution of number of hidden files by depth level in directory
    /// hierarchy

    pub fn get_num_hidden_files_by_depth(&self) -> Vec<u32> {
        if let Some(ref files) = self.files {
            let deepest = Self::deepest_depth(files);
            let mut depth_distri = vec![0u32; deepest];
            files
                .iter()
                .filter(|f| f.file_name().to_str().unwrap().starts_with("."))
                .for_each(|f| {
                    if f.depth() > 0 {
                        depth_distri[f.depth() - 1] += 1
                    }
                });
            depth_distri
        } else {
            vec![0]
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
