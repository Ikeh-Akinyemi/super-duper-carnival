use leptos::*;

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (x, set_x) = create_signal(cx, 0);
    let (y, set_y) = create_signal(cx, 0);

    let double_count = move || count() * 2;

    view! { cx,
        <div
        style="position: absolute"
        style:left=move || format!("{}px", x() + 100)
        style:top=move || format!("{}px", y() + 100)
        style:background-color=move || format!("rgb({}, {}, 150)", x(), y())
        style=("--column", x)
        >
            <p>"Move when coordinates change"</p>
            <button
                class:red=move || count() % 2 == 1
                on:click=move|_| {
                    set_count.update(|n| *n += 1);
                }
            > 
                "Click me: "
                {count}
            </button>
            <progress max="50" value=count />
            <progress max="50" value=double_count />
            <p> "Double Count: " {double_count} </p>
        </div>
    }
}