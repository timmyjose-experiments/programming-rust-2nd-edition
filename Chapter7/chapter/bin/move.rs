use std::fs;
use std::io::{self, stderr, Write};
use std::path::Path;

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in src.read_dir()? {
        let entry = entry?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }

    Ok(())
}

fn main() {
    let src = Path::new("./test_src");
    let dst = Path::new("./test_dst");
    match move_all(&src, &dst) {
        Err(e) => {
            let _ = writeln!(stderr(), "error: {:?}", e);
        }
        Ok(()) => {}
    }
}
