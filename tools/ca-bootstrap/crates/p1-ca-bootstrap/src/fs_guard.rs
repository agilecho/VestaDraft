//! Ordinary nodes only. Shared by the tool and the standalone, dependency-free FS tests.
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};

#[cfg(test)]
static CRASH_WRITES: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(-1);
#[cfg(test)]
pub(crate) fn crash_after_writes(n: i64) {
    CRASH_WRITES.store(n, std::sync::atomic::Ordering::SeqCst);
}

fn invalid() -> io::Error {
    io::Error::other("unsafe-path-or-content")
}

pub fn relative(s: &str) -> io::Result<()> {
    if s.is_empty() || s.contains(['\\', '\0']) || s.split('/').any(|p| p.is_empty() || p == "." || p == "..") || Path::new(s).is_absolute() {
        return Err(invalid());
    }
    Ok(())
}

/// Require lexical, canonical absolute paths; inspect every ancestor, including the leaf.
/// An absent leaf is allowed only when its parent already exists.
pub fn absolute(p: &Path, absent_leaf: bool) -> io::Result<()> {
    let text = p.to_str().ok_or_else(invalid)?;
    if !p.is_absolute() || text.contains(['\\', '\0']) || text != "/" && text.split('/').skip(1).any(|s| s.is_empty() || s == "." || s == "..") {
        return Err(invalid());
    }
    let mut here = PathBuf::new();
    for c in p.components() {
        if !matches!(c, Component::RootDir | Component::Normal(_)) {
            return Err(invalid());
        }
        here.push(c.as_os_str());
        match fs::symlink_metadata(&here) {
            Ok(m) if m.file_type().is_symlink() || !(m.is_file() || m.is_dir()) => return Err(invalid()),
            Ok(m) if here != p && !m.is_dir() => return Err(invalid()),
            Ok(_) => (),
            Err(e) if e.kind() == io::ErrorKind::NotFound && absent_leaf && here == p => return Ok(()),
            Err(e) => return Err(e),
        }
    }
    if fs::canonicalize(p)? != p {
        return Err(invalid());
    }
    Ok(())
}

fn same(a: &fs::Metadata, b: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        a.dev() == b.dev()
            && a.ino() == b.ino()
            && a.len() == b.len()
            && a.mtime() == b.mtime()
            && a.mtime_nsec() == b.mtime_nsec()
            && a.ctime() == b.ctime()
            && a.ctime_nsec() == b.ctime_nsec()
    }
    #[cfg(not(unix))]
    {
        a.len() == b.len() && a.modified().ok() == b.modified().ok()
    }
}

pub fn read_regular(p: &Path, max: u64) -> io::Result<Vec<u8>> {
    absolute(p, false)?;
    let before = fs::symlink_metadata(p)?;
    if !before.is_file() || before.len() > max {
        return Err(invalid());
    }
    let mut options = OpenOptions::new();
    options.read(true);
    // Keep the leaf no-follow too; ancestor checks are repeated after the read.
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x100);
    }
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x20000);
    }
    let f = options.open(p)?;
    if !same(&before, &f.metadata()?) {
        return Err(invalid());
    }
    let mut bytes = Vec::new();
    f.take(max.saturating_add(1)).read_to_end(&mut bytes)?;
    absolute(p, false)?;
    if bytes.len() as u64 != before.len() || !same(&before, &fs::symlink_metadata(p)?) {
        return Err(invalid());
    }
    Ok(bytes)
}

/// Return both ordinary files and directories, so an unknown empty directory is observable.
pub fn tree(root: &Path) -> io::Result<(Vec<String>, Vec<String>)> {
    absolute(root, false)?;
    if !fs::symlink_metadata(root)?.is_dir() {
        return Err(invalid());
    }
    fn visit(root: &Path, at: &Path, depth: usize, files: &mut Vec<String>, dirs: &mut Vec<String>) -> io::Result<()> {
        if depth > 64 {
            return Err(invalid());
        }
        for e in fs::read_dir(at)? {
            let p = e?.path();
            let m = fs::symlink_metadata(&p)?;
            let s = p.strip_prefix(root).map_err(|_| invalid())?.to_str().ok_or_else(invalid)?.to_string();
            relative(&s)?;
            if m.is_file() {
                files.push(s);
            } else if m.is_dir() {
                dirs.push(s);
                visit(root, &p, depth + 1, files, dirs)?;
            } else {
                return Err(invalid());
            }
        }
        Ok(())
    }
    let (mut files, mut dirs) = (Vec::new(), Vec::new());
    visit(root, root, 0, &mut files, &mut dirs)?;
    files.sort();
    dirs.sort();
    absolute(root, false)?;
    Ok((files, dirs))
}

/// Exclusive staging write. Nothing is overwritten, including abandoned partial writes.
pub fn write_new(p: &Path, bytes: &[u8]) -> io::Result<()> {
    absolute(p, true)?;
    let mut o = OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    let mut f = o.open(p)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    if read_regular(p, bytes.len() as u64)? != bytes {
        return Err(invalid());
    }
    sync_parent(p)?;
    #[cfg(test)]
    if CRASH_WRITES.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |v| if v > 0 { Some(v - 1) } else { None }).ok()
        == Some(1)
    {
        // Abrupt real subprocess termination, after durable ordinary bytes, before the copy
        // finishes. Disabled in production, and never armed in the standalone FS suite.
        std::process::exit(75);
    }
    Ok(())
}

pub fn sync_parent(p: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        File::open(p.parent().ok_or_else(invalid)?)?.sync_all()?;
    }
    Ok(())
}

/// All staged bytes and the complete set are checked before installation.
/// An interrupted staging tree is reusable only if every existing node is a planned exact byte.
pub fn stage_tree(stage: &Path, expected: &[(String, Vec<u8>)]) -> io::Result<()> {
    absolute(stage, true)?;
    if !stage.try_exists()? {
        fs::create_dir(stage)?;
    }
    let (present, dirs) = tree(stage)?;
    let wanted: std::collections::BTreeMap<_, _> = expected.iter().map(|(p, b)| (p.clone(), b)).collect();
    if wanted.len() != expected.len() {
        return Err(invalid());
    }
    let mut allowed_dirs = std::collections::BTreeSet::new();
    for path in wanted.keys() {
        relative(path)?;
        for parent in Path::new(path).ancestors().skip(1).filter(|p| !p.as_os_str().is_empty()) {
            allowed_dirs.insert(parent.to_str().ok_or_else(invalid)?.to_string());
        }
    }
    if dirs.iter().any(|p| !allowed_dirs.contains(p)) {
        return Err(invalid());
    }
    for p in present {
        let bytes = wanted.get(&p).ok_or_else(invalid)?;
        if read_regular(&stage.join(p), bytes.len() as u64)? != **bytes {
            return Err(invalid());
        }
    }
    for (path, bytes) in expected {
        let target = stage.join(path);
        if target.try_exists()? {
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
            absolute(parent, false)?;
        }
        write_new(&target, bytes)?;
    }
    compare_tree(stage, expected)
}

pub fn compare_tree(root: &Path, expected: &[(String, Vec<u8>)]) -> io::Result<()> {
    let (files, dirs) = tree(root)?;
    let mut names: Vec<_> = expected.iter().map(|(p, _)| p.clone()).collect();
    names.sort();
    if files != names {
        return Err(invalid());
    }
    let wanted_dirs: std::collections::BTreeSet<String> =
        names.iter().flat_map(|p| Path::new(p).ancestors().skip(1).filter(|p| !p.as_os_str().is_empty()).map(|p| p.to_string_lossy().into_owned())).collect();
    if dirs.into_iter().collect::<std::collections::BTreeSet<_>>() != wanted_dirs {
        return Err(invalid());
    }
    for (name, bytes) in expected {
        if read_regular(&root.join(name), bytes.len() as u64)? != *bytes {
            return Err(invalid());
        }
    }
    Ok(())
}

pub fn commit_tree(stage: &Path, target: &Path, expected: &[(String, Vec<u8>)]) -> io::Result<()> {
    compare_tree(stage, expected)?;
    absolute(target, true)?;
    // Reserve the directory exclusively. std::fs::rename can replace a concurrently created
    // empty directory on Unix; create_dir + exclusive file writes never does that.
    if target.try_exists()? {
        return Err(invalid());
    }
    fs::create_dir(target)?;
    stage_tree(target, expected)?;
    sync_parent(target)?;
    compare_tree(target, expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    fn root() -> PathBuf {
        let base = std::env::var_os("P2_TEST_ROOT").expect("owned explicit test root");
        let p = PathBuf::from(base).join(format!("fs-{}-{}", std::process::id(), SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst)));
        fs::create_dir(&p).unwrap();
        p
    }
    fn bytes() -> Vec<(String, Vec<u8>)> {
        vec![("nested/晶胞.json".into(), b"{\"a\":4,\"volume\":64}".to_vec()), ("raw.bin".into(), vec![0, 255, 13, 10])]
    }
    #[test]
    fn complete_copy_preserves_all_bytes_and_independent_read() {
        let r = root();
        let s = r.join("stage");
        let d = r.join("dest");
        stage_tree(&s, &bytes()).unwrap();
        commit_tree(&s, &d, &bytes()).unwrap();
        assert_eq!(fs::read(d.join("raw.bin")).unwrap(), [0, 255, 13, 10]);
        assert_eq!(fs::read_to_string(d.join("nested/晶胞.json")).unwrap(), "{\"a\":4,\"volume\":64}");
        assert!(s.exists()); // Keep the exact owned staging input as recovery evidence.
        fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn interrupted_copy_resumes_only_exact_owned_members() {
        let r = root();
        let s = r.join("stage");
        fs::create_dir(&s).unwrap();
        write_new(&s.join("raw.bin"), &[0, 255, 13, 10]).unwrap();
        stage_tree(&s, &bytes()).unwrap();
        compare_tree(&s, &bytes()).unwrap();
        fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn bad_partial_bytes_and_unknown_file_are_refused_without_overwrite() {
        let r = root();
        let s = r.join("stage");
        fs::create_dir(&s).unwrap();
        write_new(&s.join("raw.bin"), b"bad").unwrap();
        assert!(stage_tree(&s, &bytes()).is_err());
        assert_eq!(fs::read(s.join("raw.bin")).unwrap(), b"bad");
        fs::remove_file(s.join("raw.bin")).unwrap();
        write_new(&s.join("unknown"), b"owner").unwrap();
        assert!(stage_tree(&s, &bytes()).is_err());
        assert_eq!(fs::read(s.join("unknown")).unwrap(), b"owner");
        fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn unknown_empty_directory_and_missing_file_fail_complete_verification() {
        let r = root();
        let s = r.join("stage");
        stage_tree(&s, &bytes()).unwrap();
        fs::create_dir(s.join("unknown")).unwrap();
        assert!(compare_tree(&s, &bytes()).is_err());
        fs::remove_dir(s.join("unknown")).unwrap();
        fs::remove_file(s.join("raw.bin")).unwrap();
        assert!(compare_tree(&s, &bytes()).is_err());
        fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn existing_root_and_destination_never_get_replaced() {
        let r = root();
        let s = r.join("stage");
        let d = r.join("dest");
        stage_tree(&s, &bytes()).unwrap();
        fs::create_dir(&d).unwrap();
        write_new(&d.join("keep"), b"user").unwrap();
        assert!(commit_tree(&s, &d, &bytes()).is_err());
        assert_eq!(fs::read(d.join("keep")).unwrap(), b"user");
        assert!(s.exists());
        assert!(write_new(&d.join("keep"), b"wrong").is_err());
        fs::remove_dir_all(r).unwrap();
    }
    #[test]
    fn invalid_relative_and_absolute_paths_fail() {
        for s in ["", "../x", "a/../b", "a//b", "a/./b", "/a", "a\\b", "a/", "a\0b"] {
            assert!(relative(s).is_err(), "{s:?}");
        }
        assert!(absolute(Path::new("relative"), true).is_err());
        assert!(absolute(Path::new("/Users/../etc"), false).is_err());
        relative("中文/数据.bin").unwrap();
    }
    #[test]
    fn verification_is_read_only_and_changed_same_length_bytes_fail() {
        let r = root();
        let d = r.join("dest");
        stage_tree(&d, &bytes()).unwrap();
        let before = tree(&d).unwrap();
        let raw = fs::read(d.join("raw.bin")).unwrap();
        compare_tree(&d, &bytes()).unwrap();
        assert_eq!(tree(&d).unwrap(), before);
        assert_eq!(fs::read(d.join("raw.bin")).unwrap(), raw);
        fs::write(d.join("raw.bin"), [1, 255, 13, 10]).unwrap();
        assert!(compare_tree(&d, &bytes()).is_err());
        assert_eq!(fs::read(d.join("raw.bin")).unwrap(), [1, 255, 13, 10]);
        fs::remove_dir_all(r).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn symlink_leaf_ancestor_and_tree_are_rejected() {
        use std::os::unix::fs::symlink;
        let r = root();
        write_new(&r.join("source"), b"read me").unwrap();
        symlink(r.join("source"), r.join("link")).unwrap();
        assert!(read_regular(&r.join("link"), 100).is_err());
        assert!(tree(&r).is_err());
        fs::create_dir(r.join("dir")).unwrap();
        symlink(r.join("dir"), r.join("dir-link")).unwrap();
        assert!(absolute(&r.join("dir-link/new"), true).is_err());
        fs::remove_dir_all(r).unwrap();
    }
}
