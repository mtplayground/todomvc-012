use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/todomvc.css"/>
        <Link rel="stylesheet" href="/pkg/todomvc-common.css"/>
        <Link rel="stylesheet" href="/pkg/todomvc-app.css"/>
        <Title text="TodoMVC"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="todoapp">
            <h1>"todos"</h1>
        </section>
    }
}
