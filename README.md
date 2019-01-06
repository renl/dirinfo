dirinfo
=======

Crate `dirinfo` provides easy to use API for collecting various information about a
directory hierarchy.

To use this crate, add `dirinfo` as a dependency to your project's `Cargo.toml`:

``` toml
[dependencies]
dirinfo = { git = "https://github.com/renl/dirinfo" }
```

# Example

The following code calculates the total size in bytes of all files found in the
directory hierarchy with the root directory specified by the pull function.

```
use dirinfo::DirInfo;

println!("{}", DirInfo::new().pull(".").get_files_size());
```
