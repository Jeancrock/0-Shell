use std::fs::{self, Metadata, ReadDir};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

pub fn read_and_collect_dir(dir: &Path) -> io::Result<Vec<(String, Metadata)>> {
    let rd: ReadDir = fs::read_dir(dir)?;
    let mut out = Vec::new();
    for e in rd {
        let entry = e?;
        let name = entry.file_name().to_string_lossy().to_string();
        let md = fs::symlink_metadata(entry.path())?;
        out.push((name, md));
    }
    Ok(out)
}

pub fn print_long_line(path: PathBuf, md: &Metadata, classify: bool) -> Result<(), String> {
    let mode = md.mode();
    let perms = mode_to_rwx(mode);
    let size = md.size();
    let mtime = md.mtime();
    let name = format_name(path.file_name().unwrap().to_string_lossy().as_ref(), md, classify);

    println!(
        "{} {} {:>8} {} {}",
        perms, md.nlink(), size, mtime, name
    );
    Ok(())
}

pub fn format_name(name: &str, md: &Metadata, classify: bool) -> String {
    let mut n = name.to_string();
    if classify {
        if md.is_dir() { n.push('/'); }
        else if (md.mode() & 0o111) != 0 { n.push('*'); }
        else if md.file_type().is_symlink() { n.push('@'); }
    }
    n
}

pub fn mode_to_rwx(mode: u32) -> String {
    let map = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];
    map.iter().map(|(bit, ch)| if (mode & bit) != 0 { *ch } else { '-' }).collect()
}
