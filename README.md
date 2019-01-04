dirinfo
=======
A Rust library for generating information about a directory.

### Example

The following code will search through the /etc and all its subdirectories 
and calculate the total file size in bytes for files with the extension ".conf"
```
use dirinfo::DirInfo;

println!("{}", DirInfo::new().pull("/etc").get_files_size_by_file_ext(".conf");
```
