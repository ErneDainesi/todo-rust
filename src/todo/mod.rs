use crate::db::TodoSchema;
use leptos::*;

#[component]
pub fn Todo(cx: Scope, data: TodoSchema) -> impl IntoView {
    let id = data.id;
    let item_html_id = format!("item-{id}");
    let status = if data.status { "todo-item-done" } else { "todo-item" };
    view! {
        cx,
        <div class=status id=&item_html_id>
            <h3 class="todo-title">{ data.title }</h3>
            <p class="todo-description">{ data.description }</p>
            <div class="todo-actions">
                <button
                    class="todo-action"
                    hx-patch=format!("/{id}")
                    hx-trigger="click"
                    hx-target=format!("#{item_html_id}")
                    hx-swap="outerHTML"
                >Done</button>
                <button
                    class="todo-action todo-delete"
                    hx-delete=format!("/{id}")
                    hx-trigger="click"
                    hx-target=format!("#{item_html_id}")
                    hx-swap="outerHTML"
                >Remove</button>
            </div>
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
            <input class="todo-input" placeholder="Title" type="text" name="title"/>
            <input class="todo-input" placeholder="Description" type="text" name="description"/>
            <button type="submit">Add</button>
        </form>
    }
}

#[component]
pub fn TodosList(cx: Scope, todos: Vec<TodoSchema>) -> impl IntoView {
    let (todos, _) = create_signal::<Vec<TodoSchema>>(cx, todos);
    view! {
        cx,
        <section class="todo-list" id="list">
            <For
                each = move || todos.get()
                key = |item| item.id
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
