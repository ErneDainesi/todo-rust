use crate::db::TodoSchema;
use leptos::*;

#[component]
pub fn Todo(cx: Scope, data: TodoSchema) -> impl IntoView {
    view! {
        cx,
        <div>
            <h3>{ data.title }</h3>
            <p>{ data.description }</p>
        </div>
    }
}

#[component]
pub fn TodosForm(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <form
            class="todo-form"
            hx-post="/"
            hx-target="#list"
            hx-trigger="submit"
            hx-swap="beforeend"
        >
            <input placeholder="Title" type="text" name="title"/>
            <input placeholder="Description" type="text" name="description"/>
            <button type="submit">Add</button>
        </form>
    }
}

#[component]
pub fn TodosList(cx: Scope, todos: Vec<TodoSchema>) -> impl IntoView {
    let (todos, _) = create_signal::<Vec<TodoSchema>>(cx, todos);
    view! {
        cx,
        <section id="list">
            <For
                each = move || todos.get()
                key = |_item| 0 // TODO: add a correct id
                view = move |cx, item: TodoSchema| {
                    view! {
                        cx,
                        <Todo data = item />
                    }
                }
            />
        </section>
    }
}
