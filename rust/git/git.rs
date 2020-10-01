
//! ## Git
//!
//! `Git` is the module which provides git repository information

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "Pyc"
*
*   Pyc is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   Pyc is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with Pyc.  If not, see <http://www.gnu.org/licenses/>.
*
*/

extern crate git2;

use git2::Repository;
use std::path::{Path, PathBuf};

pub struct Git {}

impl Git {

    /// ### find_repository
    ///
    /// Find repository in the provided path
    pub fn find_repository(wrkdir: &PathBuf) -> Option<Repository> {
        let wrkdir_path: &Path = wrkdir.as_path();
        //Find repository
        match Repository::discover(wrkdir_path) {
            Ok(repo) => Some(repo),
            Err(_) => None,
        }
    }

    /// ### get_branch
    ///
    /// Get current branch from provided repository
    pub fn get_branch(repository: &Repository) -> Option<String> {
        let git_head = match repository.head() {
            Ok(head) => head,
            Err(_) => return None,
        };
        let shorthand = git_head.shorthand();
        shorthand.map(std::string::ToString::to_string)
    }

    /// ### get_commit
    ///
    /// Get current commit
    pub fn get_commit(repository: &Repository, hashlen: usize) -> Option<String> {
        let git_head = match repository.head() {
            Ok(head) => head,
            Err(_) => return None,
        };
        let head_commit = match git_head.peel_to_commit() {
            Ok(cmt_res) => cmt_res,
            Err(_) => return None,
        };
        let commit_oid = head_commit.id();
        Some(bytes_to_hexstr(commit_oid.as_bytes(), hashlen))
    }

    /// ### bytes_to_hexstr
    ///
    /// Convert bytes to hex string representation
    fn bytes_to_hexstr(bytes: &[u8], len: usize) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
            .chars()
            .take(len)
            .collect()
    }

}

//@! Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prompt_git_module_empty() {
        //Create temp directory
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        Repository::init(tmpdir.path()).unwrap();
        //Initialize module
        let path_str: PathBuf = PathBuf::from(tmpdir.path());
        let repo: Repository = Git::find_repository(&path_str).unwrap();
        //Branch should be none
        assert!(Git::get_branch(&repo).is_none());
        //Commit should be None
        assert!(Git::get_commit(&repo, 8).is_none());
    }

    #[test]
    fn test_prompt_git_module_with_commits() {
        /*
        //Create temp directory
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let repo: Repository = Repository::init(tmpdir.path()).unwrap();
        //Write a file
        let path_str: String = String::from(tmpdir.path().to_str().unwrap());
        let readme: String = String::from(format!("{}/README.md", path_str.clone()));
        let mut file = File::create(readme.clone()).unwrap();
        assert!(file.write_all(b"# Test repository\n\nThis is a test repository\n").is_ok());
        //Add file
        repo.
        */
        //Initialize module
        let repo: Repository = Git::find_repository(&PathBuf::from("./")).unwrap();
        //Branch should be none
        let branch = Git::get_branch(&repo);
        assert!(branch.is_some());
        println!("Current branch {}", branch.unwrap());
        //Commit should not be None
        let commit = Git::get_commit(&repo, 8);
        assert!(commit.is_some());
        println!("Current commit {}", commit.as_ref().unwrap());
        assert_eq!(commit.unwrap().len(), 8);
    }

    #[test]
    fn test_prompt_git_repo_not_found() {
        assert!(Git::find_repository(&PathBuf::from("/")).is_none());
    }
}
