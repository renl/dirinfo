use super::{BlockSize, DirInfo};

#[test]
fn filesizedistribydepth() {
    println!("{:#?}", DirInfo::new().pull(".").get_files_size_by_depth());
}

#[test]
fn distribydepthfiles() {
    println!("{:#?}", DirInfo::new().pull(".").get_num_files_by_depth());
}

#[test]
fn distribydepthdirectories() {
    println!("{:#?}", DirInfo::new().pull(".").get_num_directories_by_depth());
}

#[test]
fn distribydepthsymlinks() {
    println!("{:#?}", DirInfo::new().pull(".").get_num_symlinks_by_depth());
}

#[test]
fn getdeepest() {
    println!("{}", DirInfo::new().pull(".").get_deepest_depth());
}

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
