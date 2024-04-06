use wasm::SimpleCounter;

fn main() {
    leptos::mount_to_body(|| {
        leptos::view! {
            <SimpleCounter
                inital_value = 0
                step = 1
            />
        }
    })
}
