
use std::fs::{self, Metadata, ReadDir};
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

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

pub fn format_name(name: &str, meta: &Metadata, classify: bool) -> String {
    let mut s = name.to_string();

    if meta.is_dir() {
        s = format!("\x1b[34m{}\x1b[0m", s); // bleu pour dossiers
        if classify {
            s.push('/');
        }
    } else if meta.permissions().mode() & 0o111 != 0 {
        s = format!("\x1b[32m{}\x1b[0m", s); // vert pour exécutables
        if classify {
            s.push('*');
        }
    } else if classify && meta.is_file() {
        s.push(' ');
    }

    s
}

pub fn format_permissions(md: &Metadata) -> String {
    let mode = md.mode();
    let file_type = if md.is_dir() {
        'd'
    } else if md.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };

    let perms_map = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    let perms: String = perms_map
        .iter()
        .map(|(bit, ch)| if (mode & bit) != 0 { *ch } else { '-' })
        .collect();

    format!("{}{}", file_type, perms)
}