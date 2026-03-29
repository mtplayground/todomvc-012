use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::TodoApp;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/todomvc.css"/>
        <Title text="TodoMVC"/>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=TodoApp/>
                    <Route path="/active" view=TodoApp/>
                    <Route path="/completed" view=TodoApp/>
                </Routes>
            </main>
        </Router>
    }
}
