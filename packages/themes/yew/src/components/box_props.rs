use std::fmt::{self, Display};

use crate::props::prop_def::{prop_enum, prop_optional_responsive_enum, StringValue};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BoxAs {
    #[default]
    Div,
    Span,
}

impl Display for BoxAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxAs::Div => "div",
                BoxAs::Span => "span",
            }
        )
    }
}

impl From<BoxAs> for StringValue {
    fn from(value: BoxAs) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_enum!(BoxAsProp, BoxAs, None, None);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BoxDisplay {
    None,
    Inline,
    InlineBlock,
    Block,
}

impl Display for BoxDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxDisplay::None => "none",
                BoxDisplay::Inline => "inline",
                BoxDisplay::InlineBlock => "inline-block",
                BoxDisplay::Block => "block",
            }
        )
    }
}

impl From<BoxDisplay> for StringValue {
    fn from(value: BoxDisplay) -> Self {
        StringValue::Defined(value.to_string())
    }
}

prop_optional_responsive_enum!(BoxDisplayProp, BoxDisplay, Some("rt-r-display"), None);