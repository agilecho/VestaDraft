//! Native engineering fixture I/O, separate from platform SpaceFiles/Saved settlement.
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use vesta_codecs::SaveBundle;
use vesta_model::{Error, Result, digest};
const MAX: usize = 16 * 1024 * 1024;
#[derive(Clone)]
pub struct Paths {
    root: PathBuf,
}

pub enum PublishError {
    NotPublished(Error),
    CommitUnknown(Error),
}

impl From<Error> for PublishError {
    fn from(error: Error) -> Self {
        Self::NotPublished(error)
    }
}

impl From<PublishError> for Error {
    fn from(error: PublishError) -> Self {
        match error {
            PublishError::NotPublished(error) | PublishError::CommitUnknown(error) => error,
        }
    }
}

impl Paths {
    pub fn new(root: &str) -> Result<Self> {
        let p = Path::new(root);
        if fs::symlink_metadata(p)
            .map_err(io)?
            .file_type()
            .is_symlink()
        {
            return Err(Error::new("E_READ_ONLY", "root symlink"));
        }
        let root = fs::canonicalize(p).map_err(io)?;
        if !root.is_dir() {
            return Err(Error::new("E_IO", "root must already be a directory"));
        }
        Ok(Self { root })
    }
    pub fn resolve(&self, input: &str, create_parents: bool) -> Result<PathBuf> {
        let p = Path::new(input);
        let rel = if p.is_absolute() {
            p.strip_prefix(&self.root)
                .map_err(|_| Error::new("E_READ_ONLY", "path outside mounted root"))?
        } else {
            p
        };
        let mut out = self.root.clone();
        let parts: Vec<_> = rel.components().collect();
        if parts.is_empty() {
            return Err(Error::new("E_READ_ONLY", "file path required"));
        }
        for (i, c) in parts.iter().enumerate() {
            let Component::Normal(name) = c else {
                return Err(Error::new(
                    "E_READ_ONLY",
                    "only literal normal root-relative components allowed",
                ));
            };
            let name = name
                .to_str()
                .ok_or_else(|| Error::new("E_READ_ONLY", "UTF8 path required"))?;
            if name.is_empty() || name.contains(['\0', '\n', '\r', '\\']) {
                return Err(Error::new("E_READ_ONLY", "unsafe component"));
            }
            out.push(name);
            match fs::symlink_metadata(&out) {
                Ok(m) => {
                    if m.file_type().is_symlink() || i + 1 < parts.len() && !m.is_dir() {
                        return Err(Error::new("E_READ_ONLY", "symlink/non-directory ancestor"));
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    if i + 1 < parts.len() && create_parents {
                        fs::create_dir(&out).map_err(io)?
                    } else if i + 1 < parts.len() {
                        return Err(io(e));
                    }
                }
                Err(e) => return Err(io(e)),
            }
        }
        Ok(out)
    }
    pub fn read(&self, path: &str) -> Result<Vec<u8>> {
        self.read_with_limit(path, MAX)?
            .ok_or_else(|| Error::new("E_IO", format!("file missing: {path}")))
    }
    pub fn graph_key(&self, path: &str) -> Result<String> {
        self.resolve(path, false)?
            .strip_prefix(&self.root)
            .ok()
            .and_then(Path::to_str)
            .map(str::to_owned)
            .ok_or_else(|| Error::new("E_READ_ONLY", "graph root is outside the mounted root"))
    }
    /// Creates a native read capability for one explicitly selected existing job directory.
    /// Its absolute path stays process-local. The bundle only stores the virtual file names.
    pub fn producer_job(&self, path: &str, input: &str) -> Result<(Self, String)> {
        let directory = if Path::new(path) == self.root {
            self.root.clone()
        } else {
            self.resolve(path, false)?
        };
        let attrs = fs::symlink_metadata(&directory).map_err(io)?;
        if !attrs.is_dir() || attrs.file_type().is_symlink() {
            return Err(Error::new(
                "E_READ_ONLY",
                "producer job root must be an existing directory",
            ));
        }
        let input = self.resolve(input, false)?;
        let relative = input
            .strip_prefix(&directory)
            .ok()
            .and_then(Path::to_str)
            .filter(|path| !path.is_empty())
            .ok_or_else(|| {
                Error::new("E_READ_ONLY", "source is outside the selected producer job")
            })?;
        Ok((Self { root: directory }, relative.into()))
    }
    pub fn read_with_limit(&self, path: &str, limit: usize) -> Result<Option<Vec<u8>>> {
        let p = self.resolve(path, false)?;
        match fs::symlink_metadata(&p) {
            Ok(m) if m.is_file() => {}
            Ok(_) => return Err(Error::new("E_READ_ONLY", "source must be a regular file")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(io(e)),
        }
        let mut f = fs::File::open(&p).map_err(io)?;
        let metadata = f.metadata().map_err(io)?;
        let limit = limit.min(MAX);
        if !metadata.is_file() || metadata.len() > limit as u64 {
            return Err(Error::new("E_RESOURCE_LIMIT", "file size"));
        }
        let mut b = vec![];
        std::io::Read::by_ref(&mut f)
            .take((limit + 1) as u64)
            .read_to_end(&mut b)
            .map_err(io)?;
        if b.len() > limit {
            return Err(Error::new("E_RESOURCE_LIMIT", "read grew beyond budget"));
        }
        Ok(Some(b))
    }
    pub fn baseline(&self, path: &str) -> Result<Option<String>> {
        let p = self.resolve(path, false)?;
        match fs::metadata(&p) {
            Ok(_) => Ok(Some(digest(&self.read(path)?))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(io(e)),
        }
    }
    pub fn bundle(&self, input: &str, primary: &[u8]) -> Result<BTreeMap<String, Vec<u8>>> {
        let path = self.resolve(input, false)?;
        let leaf = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::new("E_IO", "leaf"))?;
        let parent = path.parent().ok_or_else(|| Error::new("E_IO", "parent"))?;
        let manifest_key = format!("{leaf}.resources/manifests/{}.json", digest(primary));
        let full = parent.join(&manifest_key);
        let mb = self.read(
            full.to_str()
                .ok_or_else(|| Error::new("E_IO", "manifest path"))?,
        )?;
        let manifest = vesta_model::json::parse(&mb, MAX)?;
        let mut deps = BTreeMap::new();
        let mut total = primary.len() + mb.len();
        for key in manifest["dependencies"]
            .as_object()
            .ok_or_else(|| Error::new("E_SCHEMA", "manifest dependencies"))?
            .keys()
        {
            if !key.starts_with(&format!("{leaf}.resources/")) {
                return Err(Error::new(
                    "E_READ_ONLY",
                    "dependency belongs to another root",
                ));
            }
            let full = parent.join(key);
            let b = self.read(
                full.to_str()
                    .ok_or_else(|| Error::new("E_IO", "dependency UTF8"))?,
            )?;
            total = total
                .checked_add(b.len())
                .ok_or_else(|| Error::new("E_RESOURCE_LIMIT", "bundle overflow"))?;
            if total > MAX {
                return Err(Error::new("E_RESOURCE_LIMIT", "bundle read"));
            }
            deps.insert(key.clone(), b);
        }
        deps.insert(manifest_key, mb);
        Ok(deps)
    }
    pub fn publish(
        &self,
        output: &str,
        bundle: &SaveBundle,
        expected: Option<&str>,
        nonce: &str,
    ) -> std::result::Result<String, PublishError> {
        let target = self.resolve(output, true)?;
        let parent = target
            .parent()
            .ok_or_else(|| Error::new("E_IO", "parent"))?;
        let leaf = target
            .file_name()
            .and_then(|v| v.to_str())
            .ok_or_else(|| Error::new("E_IO", "leaf"))?;
        if self.baseline(output)?.as_deref() != expected {
            return Err(Error::new(
                "E_EXTERNAL_CHANGE",
                "engineering target baseline changed",
            ).into());
        }
        for (key, bytes) in &bundle.dependencies {
            if !key.starts_with(&format!("{leaf}.resources/")) {
                return Err(Error::new("E_READ_ONLY", "bundle output root").into());
            }
            let path = parent.join(key);
            let p = self.resolve(
                path.to_str().ok_or_else(|| Error::new("E_IO", "path"))?,
                true,
            )?;
            match fs::read(&p) {
                Ok(old) => {
                    if old != *bytes {
                        return Err(Error::new(
                            "E_EXTERNAL_CHANGE",
                            "immutable resource collision",
                        ).into());
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    let mut f = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&p)
                        .map_err(io)?;
                    f.write_all(bytes).map_err(io)?;
                    f.sync_all().map_err(io)?;
                    if fs::read(&p).map_err(io)? != *bytes {
                        return Err(Error::new("E_IO", "resource readback differs").into());
                    }
                }
                Err(e) => return Err(io(e).into()),
            }
        }
        let temp = parent.join(format!(".p203-{nonce}.tmp"));
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)
            .map_err(io)?;
        let operation = (|| -> std::result::Result<_, PublishError> {
            f.write_all(&bundle.primary).map_err(io)?;
            f.sync_all().map_err(io)?;
            if fs::read(&temp).map_err(io)? != bundle.primary {
                return Err(Error::new("E_IO", "staged primary readback differs").into());
            }
            if self.baseline(output)?.as_deref() != expected {
                return Err(Error::new(
                    "E_EXTERNAL_CHANGE",
                    "primary changed before rename",
                ).into());
            }
            self.resolve(output, false)?;

            let commit = (|| {
                fs::rename(&temp, &target).map_err(io)?;
                fs::File::open(parent).map_err(io)?.sync_all().map_err(io)?;
                if fs::read(&target).map_err(io)? != bundle.primary {
                    return Err(Error::new("E_IO", "primary actual readback differs"));
                }
                Ok(digest(&bundle.primary))
            })();
            commit.map_err(PublishError::CommitUnknown)
        })();
        if temp.exists() {
            let _ = fs::remove_file(&temp);
        }
        operation
    }
}
fn io(e: std::io::Error) -> Error {
    Error::new(
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            "E_READ_ONLY"
        } else {
            "E_IO"
        },
        e.to_string(),
    )
}
