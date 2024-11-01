use yew::prelude::*;

use crate::{
    components::{
        link_props::LinkUnderlineProp,
        text::{Text, TextChildProps},
        text_props::TextSizeProp,
    },
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        leading_trim_prop::LeadingTrimProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        text_wrap_prop::TextWrapProp,
        truncate_prop::TruncateProp,
        weight_prop::WeightProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct LinkProps {
    #[prop_or_default]
    pub size: TextSizeProp,
    #[prop_or_default]
    pub weight: WeightProp,
    #[prop_or_default]
    pub trim: LeadingTrimProp,
    #[prop_or_default]
    pub truncate: TruncateProp,
    #[prop_or_default]
    pub wrap: TextWrapProp,
    #[prop_or_default]
    pub underline: LinkUnderlineProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

    // Attributes from `a`
    #[prop_or_default]
    pub download: Option<String>,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub hreflang: Option<String>,
    #[prop_or_default]
    pub ping: Option<String>,
    #[prop_or_default]
    pub referrerpolicy: Option<String>,
    #[prop_or_default]
    pub rel: Option<String>,
    #[prop_or_default]
    pub target: Option<String>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<LinkChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct LinkChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Style,
    pub download: Option<String>,
    pub href: Option<String>,
    pub hreflang: Option<String>,
    pub ping: Option<String>,
    pub referrerpolicy: Option<String>,
    pub rel: Option<String>,
    pub target: Option<String>,
    pub onclick: Callback<MouseEvent>,

    pub data_accent_color: String,
}

impl LinkChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <a
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style.to_string()}
                download={self.download}
                href={self.href}
                hreflang={self.hreflang}
                ping={self.ping}
                referrerpolicy={self.referrerpolicy}
                rel={self.rel}
                target={self.target}
                onclick={self.onclick}

                data-accent-color={self.data_accent_color}
            >
                {children}
            </a>
        }
    }
}

#[function_component]
pub fn Link(props: &LinkProps) -> Html {
    let (class, style) = extract_props(
        &[&props.color, &props.underline],
        props.class.clone(),
        props.style.clone().into(),
    );

    html! {
        <Text
            size={props.size.clone()}
            weight={props.weight.clone()}
            trim={props.trim.clone()}
            truncate={props.truncate.clone()}
            wrap={props.wrap.clone()}
            high_contrast={props.high_contrast.clone()}
            m={props.m.clone()}
            mx={props.mx.clone()}
            my={props.my.clone()}
            mt={props.mt.clone()}
            mr={props.mr.clone()}
            mb={props.mb.clone()}
            ml={props.ml.clone()}

            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-reset", &"rt-Link", &class])}
            style={style}

            as_child={Callback::from({
                let color = props.color.clone();
                let download = props.download.clone();
                let href = props.href.clone();
                let hreflang = props.hreflang.clone();
                let ping = props.ping.clone();
                let referrerpolicy = props.referrerpolicy.clone();
                let rel = props.rel.clone();
                let target = props.target.clone();
                let on_click = props.on_click.clone();
                let as_child = props.as_child.clone();
                let children = props.children.clone();

                move |TextChildProps { node_ref, id, class, style, .. }| {
                    let child_props = LinkChildProps {
                        node_ref,
                        id,
                        class,
                        style,
                        download: download.clone(),
                        href: href.clone(),
                        hreflang: hreflang.clone(),
                        ping: ping.clone(),
                        referrerpolicy: referrerpolicy.clone(),
                        rel: rel.clone(),
                        target: target.clone(),
                        onclick: on_click.clone(),

                        data_accent_color: color.0.map(|color| color.to_string()).unwrap_or("".to_string()),
                    };

                    if let Some(as_child) = as_child.as_ref() {
                        as_child.emit(child_props)
                    } else {
                        child_props.render(children.clone())
                    }
                }
            })}
        />
    }
}