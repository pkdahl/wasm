use leptos::{
    IntoView,
    SignalGet,
    SignalSet,
    SignalUpdate,
    component,
    create_signal,
    view,
};
// use leptos::*;

#[component]
pub fn SimpleCounter(inital_value: i32, step: i32) -> impl IntoView {
    let (value, set_value) = create_signal(inital_value);

    view! {
        <div>
            <button on:click=move |_| set_value.set(0)>"Clear"</button><br />
            <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
            <span>"Value: " {move || value.get()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
        </div>
    }
}
