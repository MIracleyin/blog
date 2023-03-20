use yew::prelude::*;

use crate::components::card::Card;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    use_context::<Callback<String>>()
    .unwrap()
    .emit("找不到页面".into());

html! {
    <Card title={"Welcome!"}>
        <p>{ "404 NOT FOUND，换个地址试试看？" }</p>
    </Card>
}
}