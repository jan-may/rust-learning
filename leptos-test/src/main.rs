use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    let (toggled, set_toggled) = create_signal(false);

    // share `set_toggled` with all children of this component
    provide_context(set_toggled);

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me my dudes"
        </button>
        <ProgressBar progress=count/>
        // add a second progress bar
        <ProgressBar progress=double_count/>
        <StaticList/>
        <DynamicList/>
         <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Control Flow"</h2>
        <Controll_Flow value=count/>
        <h2>"Type Conversion"</h2>
        <Type_Conversion value=count/>
        <h2>"Numeric Input"</h2>
        <NumericInput/>
        <p>"Toggled? " {toggled}</p>
        <Layout/>
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
    let (data, _set_data) = create_signal(vec![0, 1, 2]);
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

#[component]
fn ControlledComponent() -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //
            // IMPORTANT: the `value` *attribute* only sets the
            // initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; I didn't invent it.
            // Other frameworks gloss this over; I think it's
            // more important to give you access to the browser
            // as it really works.
            //
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn Controll_Flow(value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;
    let message = move || match value() {
        0 => "Zero",
        1 => "One",
        _n if is_odd() => "Odd",
        _ => "Even",
    };
    view! {
        <p>{value} = {message}</p>
    }
}

#[component]
fn Type_Conversion(value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;
    view! {
        <main>
            {move || match is_odd() {
                true if value() == 1 => {
                    // returns HtmlElement<Pre>
                    view! { <pre>"One"</pre> }.into_any()
                },
                false if value() == 2 => {
                    // returns HtmlElement<P>
                    view! { <p>"Two"</p> }.into_any()
                }
                // returns HtmlElement<Textarea>
                _ => view! { <textarea>{value()}</textarea> }.into_any()
            }}
        </main>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

// Context API
#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    // use_context searches up the context tree, hoping to
    // find a `WriteSignal<bool>`
    // in this case, I .expect() because I know I provided it
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
