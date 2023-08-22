use leptos::*;

#[component]
pub fn Todo(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
            <h3>This will be a todo item</h3>
            <p>bla bla bla...</p>
        </div>
    }
}
