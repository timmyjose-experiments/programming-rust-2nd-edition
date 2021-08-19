use std::error::Error;
use std::path::Path;

fn list_curr_dir() -> Result<(), Box<dyn Error>> {
    let curr_dir = Path::from(".");

    for entry in curr_dir.read_dir()? {
        let entry = entry?;
        println!(
            "file name = {}, path = {}, file type = {:?}",
            entry.file_name().to_string_lossy(),
            entry.path().to_string_lossy(),
            entry.file_type()?
        );
    }

    Ok(())
}

fn main() {
    list_curr_dir().unwrap();
}