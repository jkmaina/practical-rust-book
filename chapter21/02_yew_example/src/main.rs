use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };
    
    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter - 1);
        })
    };
    
    let reset = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(0);
        })
    };
    
    html! {
        <div>
            <h1>{ "🦀 Yew Counter Example" }</h1>
            <div class="counter">
                { "Count: " }{ *counter }
            </div>
            <div>
                <button onclick={increment}>{ "➕ Increment" }</button>
                <button onclick={decrement}>{ "➖ Decrement" }</button>
                <button class="reset" onclick={reset}>{ "🔄 Reset" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}