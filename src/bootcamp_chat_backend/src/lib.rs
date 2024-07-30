use std::{cell::{Ref, RefCell}, collections::HashMap};
use candid::Principal


thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn get_notes(user: Principal) -> Vec<String> {
    NOTES.with_borrow(|notes| notes.get(user).clone())
}

#[ic_cdk::update]
fn add_note(note: String) {
    NOTES.with_borrow_mut(|notes| {
        notes.push(note)
    })
}