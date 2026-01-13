//! vision_core: shared detector/capture/overlay interfaces.

pub mod capture;
pub mod interfaces;
pub mod overlay;

pub mod prelude {
    pub use crate::capture::{
        CaptureLimit, PrimaryCaptureCamera, PrimaryCaptureReadback, PrimaryCaptureTarget,
    };
    pub use crate::interfaces::*;
    pub use crate::overlay::*;
}
