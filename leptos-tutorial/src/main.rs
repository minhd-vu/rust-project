use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);

    // derived signals
    let double_count = move || count() * 2;
    let html = "<p>This HTML will be injected.</p>";

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
                set_x.update(|n| *n += 10);
            }

            class:red=move || count() % 2 == 1
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            // we use it once here
            value=double_count
        ></progress>
        <p>
            // and again here
            "Double Count: " {double_count}
        </p>
        // injected html
        <div inner_html=html></div>
    }
}
