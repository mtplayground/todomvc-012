use leptos::*;
use crate::todo::add_todo;

#[component]
pub fn TodoInput(on_add: Callback<()>) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(String::new());

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let title = input_value.get();
        let title = title.trim().to_string();
        if title.is_empty() {
            return;
        }
        let on_add = on_add;
        set_input_value.set(String::new());
        spawn_local(async move {
            if add_todo(title).await.is_ok() {
                on_add.call(());
            }
        });
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <form on:submit=handle_submit>
                <input
                    class="new-todo"
                    placeholder="What needs to be done?"
                    autofocus
                    prop:value=input_value
                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                />
            </form>
        </header>
    }
}
