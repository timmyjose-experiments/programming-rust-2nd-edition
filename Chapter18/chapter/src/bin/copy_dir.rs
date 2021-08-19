use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry in src.read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}

fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else {
        return Err(io::Error::new(
            ErrorKind::Other,
            format!("don't know how to copy {}", src.display()),
        ));
    }

    Ok(())
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: copy_dir SRC DST");
        std::process::exit(1);
    }

    let src_path = Path::new(&args[0]);
    let dst_path = Path::new(&args[1]);

    match copy_dir_to(&src_path, &dst_path) {
        Ok(()) => println!("Copy successful"),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2);
        }
    }
}
