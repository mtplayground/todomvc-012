pub mod app;
pub mod components;
pub mod todo;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::*;
    mount_to_body(app::App);
}

pub use todo::{add_todo, clear_completed, delete_todo, get_todos, toggle_todo, update_todo_title, Todo};

#[cfg(feature = "ssr")]
pub use todo::{
    db_add_todo, db_clear_completed, db_delete_todo, db_get_todos, db_toggle_all, db_toggle_todo,
    db_update_todo_title,
};
