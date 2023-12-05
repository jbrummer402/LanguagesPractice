
#[component]
pub fn TypeArea(cx: Scope, Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    
    view! { cx,
        <div>
            <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input does not exist");
                send.dispatch(input.value());
                input.set_value("");
            }>
                <input type='text' class="w-2/3 p-4 border border-gray-300 rounded-full" node_ref=input_ref/>
                <input type='submit' class="h-full p-4 bg-blue-500 text-white rounded-full cursor-pointer"/>
            </form>
        </div>
    }
}