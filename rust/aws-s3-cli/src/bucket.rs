/**
 *
 *
 *           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                   Version 2, December 2004
 *
 *  Copyright (C) 2021 Christian Visintin
 *
 *  Everyone is permitted to copy and distribute verbatim or modified
 *  copies of this license document, and changing it is allowed as long
 *  as the name is changed.
 *
 *             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
 *
 *   0. You just DO WHAT THE FUCK YOU WANT TO.
*/
use s3::creds::Credentials;
use s3::serde_types::Object as S3Object;
use s3::{Bucket, Region};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub type BucketResult<T> = Result<T, String>;

pub struct S3Bucket {
    bucket: Bucket,
    wrkdir: PathBuf,
}

#[derive(Debug)]
pub struct S3File {
    pub name: String,
    pub path: PathBuf,
    pub size: usize,
    pub last_modified: String,
    pub is_dir: bool,
}

impl S3File {
    pub fn new(obj: &S3Object) -> Self {
        let is_dir: bool = obj.key.ends_with('/');
        Self {
            name: S3Bucket::object_name(obj.key.as_str()),
            path: PathBuf::from(obj.key.as_str()),
            size: obj.size as usize,
            last_modified: obj.last_modified.clone(),
            is_dir,
        }
    }
}

impl S3Bucket {
    /// ### list_object_should_be_kept
    ///
    /// Returns whether object should be kept after list command.
    /// The object won't be kept if:
    ///
    /// 1. is not a direct child of provided dir
    fn list_object_should_be_kept(obj: &S3Object, dir: &str) -> bool {
        Self::is_direct_child(obj.key.as_str(), dir)
    }

    /// ### is_direct_child
    ///
    /// Checks whether Object's key is direct child of `parent` path.
    fn is_direct_child(key: &str, parent: &str) -> bool {
        key == format!("{}{}", parent, Self::object_name(key))
            || key == format!("{}{}/", parent, Self::object_name(key))
    }

    /// ### object_name
    ///
    /// Get object name from key
    pub fn object_name(key: &str) -> String {
        let mut tokens = key.split('/');
        let count = tokens.clone().count();
        let demi_last: String = match count > 1 {
            true => tokens.nth(count - 2).unwrap().to_string(),
            false => String::new(),
        };
        if let Some(last) = tokens.last() {
            // If last is not empty, return last one
            if !last.is_empty() {
                return last.to_string();
            }
        }
        // Return demi last
        demi_last
    }
}

impl S3Bucket {
    pub fn connect(name: &str, region: &str, profile: Option<&str>) -> BucketResult<Self> {
        // Get region
        let region: Region =
            Region::from_str(region).map_err(|e| format!("Invalid region: {}", e))?;
        // Get credentials
        let credentials: Credentials = Credentials::new(None, None, None, None, profile.as_deref())
            .map_err(|e| format!("Could not get credentials: {}", e))?;
        // Connect to bucket
        let bucket: Bucket = Bucket::new(name, region, credentials)
            .map_err(|e| format!("Could not connect to bucket: {}", e))?;
        Ok(Self {
            bucket,
            wrkdir: PathBuf::from("/"),
        })
    }

    pub fn pwd(&self) -> BucketResult<&Path> {
        Ok(self.wrkdir.as_path())
    }

    pub fn list(&self, dir: Option<&str>) -> BucketResult<Vec<S3File>> {
        let dir: String = match dir {
            Some(d) => d.to_string(),
            None => self.wrkdir.to_string_lossy().to_string(),
        };
        // If root, convert to empty string
        let dir: String = match dir.as_str() == "/" {
            true => String::default(),
            false => dir,
        };
        let results = self.bucket.list(dir.clone(), None);
        match results {
            Ok(entries) => {
                let mut objects: Vec<S3File> = Vec::new();
                entries.iter().for_each(|x| {
                    x.contents
                        .iter()
                        .filter(|x| Self::list_object_should_be_kept(x, dir.as_str()))
                        .for_each(|x| objects.push(S3File::new(x)))
                });
                Ok(objects)
            }
            Err(e) => Err(format!("Could not list dir: {}", e)),
        }
    }

    pub fn stat(&self, p: &str) -> BucketResult<S3File> {
        let results = self.bucket.list(p.to_string(), None);
        match results {
            Ok(entries) => {
                let mut objects: Vec<S3File> = Vec::new();
                entries
                    .iter()
                    .for_each(|x| x.contents.iter().for_each(|x| objects.push(S3File::new(x))));
                match objects.into_iter().find(|x| x.path == PathBuf::from(p)) {
                    Some(file) => Ok(file),
                    None => Err(format!("{}: No such file or directory", p)),
                }
            }
            Err(e) => Err(format!("Could not list dir: {}", e)),
        }
    }

    pub fn change_dir(&mut self, p: &str) -> BucketResult<()> {
        let p: String = match p.ends_with('/') {
            true => p.to_string(),
            false => format!("{}/", p),
        };
        let parent: String = format!(
            "{}/",
            PathBuf::from(p.as_str())
                .parent()
                .map(|x| x.to_path_buf())
                .unwrap_or_default()
                .display()
        );
        // List directory and catch...
        let objects = self.list(Some(parent.as_str()))?;
        let new_path = PathBuf::from(p.as_str());
        if objects.iter().any(|x| x.path == new_path && x.is_dir) {
            self.wrkdir = new_path;
            Ok(())
        } else {
            Err(format!("{}: No such file or directory", p))
        }
    }

    pub fn remove(&self, p: &str) -> BucketResult<()> {
        self.bucket
            .delete_object(p)
            .map(|_| ())
            .map_err(|e| format!("Could not remove object: {}", e))
    }

    pub fn put(&self, src: &Path, dest: &str) -> BucketResult<()> {
        let mut reader = File::open(src).map_err(|e| format!("Could not open file: {}", e))?;
        self.bucket
            .put_object_stream(&mut reader, dest)
            .map(|_| ())
            .map_err(|e| format!("Could not put file: {}", e))
    }

    pub fn get(&self, src: &str, dest: &Path) -> BucketResult<()> {
        let mut writer = File::create(dest).map_err(|e| format!("Could not open file: {}", e))?;
        self.bucket
            .get_object_stream(src, &mut writer)
            .map(|_| ())
            .map_err(|e| format!("Could not get file: {}", e))
    }

    pub fn mkdir(&self, p: &str) -> BucketResult<()> {
        // FIXME: use stat to check whether file exists
        let p: String = match p.ends_with('/') {
            true => p.to_string(),
            false => format!("{}/", p),
        };
        self.bucket
            .put_object(p.as_str(), &[])
            .map(|_| ())
            .map_err(|e| format!("Could not make directory: {}", e))
    }
}
