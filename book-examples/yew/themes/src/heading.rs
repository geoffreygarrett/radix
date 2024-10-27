#[allow(clippy::module_inception)]
#[cfg(feature = "heading")]
pub mod heading;
#[cfg(feature = "heading-align")]
pub mod heading_align;
#[cfg(feature = "heading-as")]
pub mod heading_as;
#[cfg(feature = "heading-color")]
pub mod heading_color;
#[cfg(feature = "heading-high-contrast")]
pub mod heading_high_contrast;
#[cfg(feature = "heading-size")]
pub mod heading_size;
#[cfg(feature = "heading-trim")]
pub mod heading_trim;
#[cfg(feature = "heading-trim-box")]
pub mod heading_trim_box;
#[cfg(feature = "heading-truncate")]
pub mod heading_truncate;
#[cfg(feature = "heading-weight")]
pub mod heading_weight;
#[cfg(feature = "heading-wrap")]
pub mod heading_wrap;