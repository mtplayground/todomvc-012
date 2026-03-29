use leptos::*;
use leptos_router::*;
use crate::todo::{get_todos, clear_completed};
use super::todo_input::TodoInput;
use super::todo_list::TodoList;
use super::footer::{Filter, Footer};

#[component]
pub fn TodoApp() -> impl IntoView {
    let (refresh, set_refresh) = create_signal(0u32);

    let on_add = Callback::new(move |_| {
        set_refresh.update(|n| *n += 1);
    });

    let todos = create_resource(
        move || refresh.get(),
        |_| async move { get_todos().await },
    );

    let active_count = Signal::derive(move || {
        todos.get()
            .and_then(|r| r.ok())
            .map(|list| list.iter().filter(|t| !t.completed).count())
            .unwrap_or(0)
    });

    let completed_count = Signal::derive(move || {
        todos.get()
            .and_then(|r| r.ok())
            .map(|list| list.iter().filter(|t| t.completed).count())
            .unwrap_or(0)
    });

    let total_count = Signal::derive(move || {
        todos.get()
            .and_then(|r| r.ok())
            .map(|list| list.len())
            .unwrap_or(0)
    });

    let on_clear_completed = Callback::new(move |_| {
        spawn_local(async move {
            if clear_completed().await.is_ok() {
                set_refresh.update(|n| *n += 1);
            }
        });
    });

    let location = use_location();
    let filter = Signal::derive(move || Filter::from_path(&location.pathname.get()));

    view! {
        <section class="todoapp">
            <TodoInput on_add=on_add/>
            {move || {
                if total_count.get() > 0 {
                    view! {
                        <TodoList refresh=refresh filter=filter/>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            {move || {
                if total_count.get() > 0 {
                    view! {
                        <Footer
                            active_count=active_count
                            completed_count=completed_count
                            on_clear_completed=on_clear_completed
                        />
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </section>
    }
}
