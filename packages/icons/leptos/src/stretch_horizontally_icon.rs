use leptos::{svg::Svg, *};
#[component]
pub fn StretchHorizontallyIcon(
    #[prop(default = "currentColor".into(), into)] color: MaybeSignal<String>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <svg
            {..attrs}
            node_ref=node_ref
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M14.4999 0.999992C14.2237 0.999992 13.9999 1.22385 13.9999 1.49999L13.9999 5.99995L0.999992 5.99995L0.999992 1.49999C0.999992 1.22385 0.776136 0.999992 0.499996 0.999992C0.223856 0.999992 -9.78509e-09 1.22385 -2.18556e-08 1.49999L4.07279e-07 13.4999C3.95208e-07 13.776 0.223855 13.9999 0.499996 13.9999C0.776136 13.9999 0.999992 13.776 0.999992 13.4999L0.999992 8.99992L13.9999 8.99992L13.9999 13.4999C13.9999 13.776 14.2237 13.9999 14.4999 13.9999C14.776 13.9999 14.9999 13.776 14.9999 13.4999L14.9999 1.49999C14.9999 1.22385 14.776 0.999992 14.4999 0.999992Z"
                fill=color
            />
        </svg>
    }
}