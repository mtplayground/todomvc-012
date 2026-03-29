use leptos::*;
use super::todo_input::TodoInput;
use super::todo_list::TodoList;

#[component]
pub fn TodoApp() -> impl IntoView {
    let (refresh, set_refresh) = create_signal(0u32);

    let on_add = Callback::new(move |_| {
        set_refresh.update(|n| *n += 1);
    });

    view! {
        <section class="todoapp">
            <TodoInput on_add=on_add/>
            <TodoList refresh=refresh/>
        </section>
    }
}
