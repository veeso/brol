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

use std::rc::Rc;

pub struct User {
    username: String,
    age: u8,
}

pub struct UserView {
    users: Vec<Rc<User>>,
    selected: Option<Rc<User>>,
}

impl Default for UserView {
    fn default() -> Self {
        UserView {
            users: Vec::new(),
            selected: None,
        }
    }
}

impl UserView {

    pub fn add_user(&mut self, username: String, age: u8) {
        self.users.push(
            Rc::new(User {
                username,
                age,
            })
        )
    }

    pub fn del_user(&mut self, username: &str) {
        // Remove element from view
        self.users.retain(|u| u.username.as_str() != username);
        // If selected user is `username` set to None
        if let Some(u) = self.selected.as_mut() {
            if u.username.as_str() == username {
                self.selected = None;
            }
        }
    }

    pub fn select_user(&mut self, username: &str) {
        if let Some(u) = self.users.iter().find(|&u| u.username.as_str() == username) {
            self.selected = Some(Rc::clone(&u));
        }
    }

}

fn main() -> Result<(), ()> {
    
    let mut view: UserView = UserView::default();
    // Add 2 users
    view.add_user(String::from("omar"), 32);
    view.add_user(String::from("pippo"), 27);
    assert!(view.selected.is_none());
    assert_eq!(view.users.len(), 2);
    // Select user
    view.select_user("omar");
    assert!(view.selected.is_some());
    println!("Selected user: {}, age: {}", view.selected.as_ref().unwrap().username, view.selected.as_ref().unwrap().age);
    // Remove user
    view.del_user("pippo");
    println!("Selected user: {}, age: {}", view.selected.as_ref().unwrap().username, view.selected.as_ref().unwrap().age);
    assert_eq!(view.users.len(), 1);
    assert!(view.selected.is_some());
    view.del_user("omar");
    assert!(view.selected.is_none());
    assert_eq!(view.users.len(), 0);
    println!("Selected user: None");

    Ok(())
}
