use dirs::home_dir;
use std::error::Error;
use std::fs::DirEntry;
use std::{fs, path};

fn get_files(homedir: &path::PathBuf) {
    let dirs = fs::read_dir(&homedir)
        .expect("msg")
        .filter_map(|dir| dir.ok())
        .filter(|dir| !dir.file_name().to_str().unwrap().starts_with("."))
        .filter(|dir| {
            !(dir.path().is_dir()
                && dir
                    .file_name()
                    .to_str()
                    .unwrap()
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()))
        });

    println!("{:?}", dirs);

    for dir in dirs {
        println!("{:?}", dir.path().to_str().unwrap())
    }
    // let dirs: Vec<DirEntry> = dirs.collect();
    // println!("{:?}", dirs);
}

fn move_file(homedir: &path::PathBuf, path: path::PathBuf) -> Result<(), Box<dyn Error>> {
    if path.is_file() {
        let file_name = path.file_name().unwrap();
        if file_name.to_str().unwrap().starts_with(".") {
            return Ok(());
        };

        let extension = match path.extension() {
            None => String::from("NOEXT"),
            Some(ext) => ext.to_str().unwrap().to_uppercase(),
        };
        let new_dir = homedir.join(extension);

        fs::create_dir_all(&new_dir)?;

        let new_path = &new_dir.join(file_name);

        println!(
            "Move {:?} => {:?}",
            path.strip_prefix(homedir)?.as_os_str(),
            new_path.strip_prefix(homedir)?.as_os_str(),
        );

        fs::rename(path, new_path)?;
    } else if path.is_dir() {
        let dir_name = path.file_name().unwrap();
        let dir_str = dir_name.to_str().unwrap();

        if !dir_str
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            let new_dir = homedir.join("DIRS");
            fs::create_dir_all(&new_dir)?;

            let new_path = &new_dir.join(dir_name);
            println!(
                "Move {:?} => {:?}",
                path.strip_prefix(homedir)?.as_os_str(),
                new_path.strip_prefix(homedir)?.as_os_str(),
            );
            fs::rename(path, new_path)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut homedir = home_dir().ok_or("Error")?;
    homedir.push("Downloads");

    // let dirs = fs::read_dir(&homedir)?;
    get_files(&homedir);

    // for dir in dirs {
    //     let path = dir?.path();
    //     move_file(&homedir, path).expect("Error moving file");
    // }
    Ok(())
}
// Alternative  let files = fs::read_dir("path/to/directory")?
// .filter_map(|entry| entry.ok())
// .filter(|entry| !entry.file_name().to_str().unwrap().starts_with("."));
