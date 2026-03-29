use leptos::*;
use leptos_router::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn from_path(path: &str) -> Self {
        match path {
            "/active" => Filter::Active,
            "/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Filter::All => "/",
            Filter::Active => "/active",
            Filter::Completed => "/completed",
        }
    }
}

#[component]
pub fn Footer(
    active_count: Signal<usize>,
    completed_count: Signal<usize>,
    on_clear_completed: Callback<()>,
) -> impl IntoView {
    let location = use_location();
    let current_filter = move || Filter::from_path(&location.pathname.get());

    let filter_class = move |filter: Filter| {
        if current_filter() == filter {
            "selected"
        } else {
            ""
        }
    };

    view! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{active_count}</strong>
                {move || if active_count.get() == 1 { " item left" } else { " items left" }}
            </span>
            <ul class="filters">
                <li>
                    <a href="/" class=move || filter_class(Filter::All)>
                        "All"
                    </a>
                </li>
                <li>
                    <a href="/active" class=move || filter_class(Filter::Active)>
                        "Active"
                    </a>
                </li>
                <li>
                    <a href="/completed" class=move || filter_class(Filter::Completed)>
                        "Completed"
                    </a>
                </li>
            </ul>
            {move || {
                if completed_count.get() > 0 {
                    view! {
                        <button
                            class="clear-completed"
                            on:click=move |_| on_clear_completed.call(())
                        >
                            "Clear completed"
                        </button>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </footer>
    }
}
