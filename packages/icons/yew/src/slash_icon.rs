use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SlashIconProps {
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub height: AttrValue,
}
#[function_component]
pub fn SlashIcon(props: &SlashIconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            class={&props.class}
            width={&props.width}
            height={&props.height}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M4.10876 14L9.46582 1H10.8178L5.46074 14H4.10876Z"
                fill={&
        props.color}
            />
        </svg>
    }
}
