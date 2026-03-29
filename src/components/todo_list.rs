use leptos::*;
use crate::todo::get_todos;
use super::todo_item::TodoItem;

#[component]
pub fn TodoList(refresh: ReadSignal<u32>) -> impl IntoView {
    let todos = create_resource(
        move || refresh.get(),
        |_| async move { get_todos().await },
    );

    view! {
        <section class="main">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    todos.get().map(|result| match result {
                        Ok(todo_list) => {
                            let items: Vec<_> = todo_list
                                .into_iter()
                                .map(|todo| {
                                    view! {
                                        <TodoItem
                                            todo=todo
                                            on_change=Callback::new(move |_| {
                                                todos.refetch();
                                            })
                                        />
                                    }
                                })
                                .collect();
                            view! {
                                <ul class="todo-list">{items}</ul>
                            }.into_view()
                        }
                        Err(_) => view! { <p>"Error loading todos."</p> }.into_view(),
                    })
                }}
            </Suspense>
        </section>
    }
}
