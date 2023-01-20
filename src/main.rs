use std::{fs, path};
use dirs::home_dir;
use std::error::Error;

fn move_file(homedir: &path::PathBuf, path: path::PathBuf) -> Result<(), Box<dyn Error>>{
    if path.is_file(){

        let extension = match path.extension() {
            None => String::from("NOEXT"),
            Some(ext) => ext.to_str().unwrap().to_uppercase(),
        };
        let new_dir = homedir
            .join(extension);

        fs::create_dir_all(&new_dir)?;

        let new_path = &new_dir
            .join(path.file_name().unwrap());

        println!("Move {:?} => {:?}", 
            path.strip_prefix(homedir)?.to_str().unwrap(),
            new_path.strip_prefix(homedir)?.to_str().unwrap(),
        );

        fs::rename(path, new_path)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut homedir = home_dir().ok_or("Error")?;
    homedir.push("Downloads");
    
    let dirs = fs::read_dir(&homedir)?;

    for dir in dirs {
        let path = dir?.path();
        move_file(&homedir, path).expect("Error moving file");
    }
    Ok(())
}
// Add error handling: unwrap() and expect() should be avoided in production code as they panic on errors. Instead, use match statements or the Result type to handle errors properly.
// Use as_path() instead of path() when getting the path from a DirEntry
// Use strip_prefix() only when you need to, the function is quite expensive and if you don't need to use it avoid it.
// Use path.is_file() instead of if path.is_file()
// Use path.extension() instead of match path.extension()
// Use path.file_name().unwrap() instead of &path.file_name().unwrap()
// Use fs::create_dir_all() instead of fs::create_dir_all(&new_dir).unwrap()
// Use fs::rename() instead of fs::rename(&path, &new_path).unwrap()
// Add some comments on the code explaining the logic behind it and what each function does.
// Use as_os_str() instead of to_str().unwrap()