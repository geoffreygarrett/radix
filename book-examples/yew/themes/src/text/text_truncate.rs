use radix_yew_themes::{Flex, Text};
use yew::prelude::*;

#[function_component]
pub fn TextTruncateExample() -> Html {
    html! {
        <Flex max_width="300px">
            <Text truncate=true>
                {"The goal of typography is to relate font size, line height, and line width
                in a proportional way that maximizes beauty and makes reading easier and
                more pleasant."}
            </Text>
        </Flex>
    }
}
