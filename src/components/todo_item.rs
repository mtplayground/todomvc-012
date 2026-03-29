use leptos::*;
use crate::todo::{delete_todo, toggle_todo, update_todo_title, Todo};

#[component]
pub fn TodoItem(todo: Todo, on_change: Callback<()>) -> impl IntoView {
    let id = todo.id;
    let (completed, set_completed) = create_signal(todo.completed);
    let (editing, set_editing) = create_signal(false);
    let (title, set_title) = create_signal(todo.title.clone());
    let (edit_value, set_edit_value) = create_signal(todo.title.clone());
    let input_ref = create_node_ref::<html::Input>();

    let handle_toggle = {
        let on_change = on_change;
        move |_| {
            let on_change = on_change;
            set_completed.update(|c| *c = !*c);
            spawn_local(async move {
                if toggle_todo(id).await.is_ok() {
                    on_change.call(());
                } else {
                    set_completed.update(|c| *c = !*c);
                }
            });
        }
    };

    let handle_delete = {
        let on_change = on_change;
        move |_| {
            let on_change = on_change;
            spawn_local(async move {
                if delete_todo(id).await.is_ok() {
                    on_change.call(());
                }
            });
        }
    };

    let start_editing = move |_| {
        // Sync edit_value with the current committed title before entering edit mode
        set_edit_value.set(title.get());
        set_editing.set(true);
        // Focus the input after it renders
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };

    // Called on blur or Enter key — save the edit
    let commit_edit = {
        let on_change = on_change;
        move || {
            let trimmed = edit_value.get();
            let trimmed = trimmed.trim().to_string();
            set_editing.set(false);
            // Update the displayed title optimistically
            set_title.set(trimmed.clone());
            // Reset edit_value to match the new committed title
            set_edit_value.set(trimmed.clone());
            let on_change = on_change;
            spawn_local(async move {
                if update_todo_title(id, trimmed).await.is_ok() {
                    on_change.call(());
                }
            });
        }
    };

    let handle_keydown = {
        let commit_edit = commit_edit.clone();
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            if key == "Enter" {
                commit_edit();
            } else if key == "Escape" {
                // Cancel: restore committed title and leave edit mode
                set_edit_value.set(title.get());
                set_editing.set(false);
            }
        }
    };

    let handle_blur = move |_| {
        if editing.get() {
            commit_edit();
        }
    };

    // After editing becomes true, focus the input
    create_effect(move |_| {
        if editing.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });

    view! {
        <li class=move || {
            let mut classes = Vec::new();
            if completed.get() {
                classes.push("completed");
            }
            if editing.get() {
                classes.push("editing");
            }
            classes.join(" ")
        }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=move || completed.get()
                    on:change=handle_toggle
                />
                <label on:dblclick=start_editing>{
                    move || title.get()
                }</label>
                <button class="destroy" on:click=handle_delete></button>
            </div>
            <input
                node_ref=input_ref
                class="edit"
                prop:value=move || edit_value.get()
                on:input=move |ev| set_edit_value.set(event_target_value(&ev))
                on:keydown=handle_keydown
                on:blur=handle_blur
            />
        </li>
    }
}
