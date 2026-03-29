use leptos::*;
use crate::todo::{delete_todo, toggle_todo, Todo};

#[component]
pub fn TodoItem(todo: Todo, on_change: Callback<()>) -> impl IntoView {
    let id = todo.id;
    let completed = todo.completed;
    let title = todo.title.clone();

    let handle_toggle = {
        let on_change = on_change;
        move |_| {
            let on_change = on_change;
            spawn_local(async move {
                if toggle_todo(id).await.is_ok() {
                    on_change.call(());
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
        <li class=if completed { "completed" } else { "" }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=handle_toggle
                />
                <label>{title}</label>
                <button class="destroy" on:click=handle_delete></button>
            </div>
        </li>
    }
}
