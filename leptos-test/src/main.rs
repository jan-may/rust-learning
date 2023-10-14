use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        <ProgressBar progress=count/>
        // add a second progress bar
        <ProgressBar progress=double_count/>
        <StaticList/>
        <DynamicList/>
    }
}

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn StaticList() -> impl IntoView {
    let values = vec![0, 1, 2];

    view! {
    // this will just render "012"
    <p>{values.clone()}</p>
    // or we can wrap them in <li>
    <ul>
        {values.into_iter()
            .map(|n| view! { <li>{n}</li>})
            .collect_view()}
    </ul>
    }
}

#[component]
fn DynamicList() -> impl IntoView {
    let (data, set_data) = create_signal(vec![0, 1, 2]);
    view! {
        // more efficient than StaticList
        <For
            each=move || data.get()
            key=|n| *n
            // stores the item in each row in a variable named `data`
            let:data
        >
            <p>{data}</p>
        </For>
    }
}
