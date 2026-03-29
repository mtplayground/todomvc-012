use leptos::*;
use crate::todo::{delete_todo, toggle_todo, Todo};

#[component]
pub fn TodoItem(todo: Todo, on_change: Callback<()>) -> impl IntoView {
    let id = todo.id;
    let (completed, set_completed) = create_signal(todo.completed);

    let handle_toggle = {
        let on_change = on_change;
        move |_| {
            let on_change = on_change;
            // Optimistically flip the local completed state for immediate feedback
            set_completed.update(|c| *c = !*c);
            spawn_local(async move {
                if toggle_todo(id).await.is_ok() {
                    on_change.call(());
                } else {
                    // Revert optimistic update on failure
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

    view! {
        <li class=move || if completed.get() { "completed" } else { "" }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=move || completed.get()
                    on:change=handle_toggle
                />
                <label style=move || {
                    if completed.get() {
                        "text-decoration: line-through;"
                    } else {
                        ""
                    }
                }>{todo.title.clone()}</label>
                <button class="destroy" on:click=handle_delete></button>
            </div>
        </li>
    }
}
