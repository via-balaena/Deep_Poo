pub mod factory;
pub mod plugin;

#[cfg(feature = "backend-wgpu")]
pub type InferenceBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type InferenceBackend = burn_ndarray::NdArray<f32>;

#[cfg(feature = "bigdet")]
pub type InferenceModel<B> = models::BigDet<B>;
#[cfg(feature = "bigdet")]
pub type InferenceModelConfig = models::BigDetConfig;
#[cfg(not(feature = "bigdet"))]
pub type InferenceModel<B> = models::TinyDet<B>;
#[cfg(not(feature = "bigdet"))]
pub type InferenceModelConfig = models::TinyDetConfig;

pub use factory::{InferenceFactory, InferenceThresholds};
pub use plugin::InferencePlugin;

pub mod prelude {
    pub use crate::factory::{InferenceFactory, InferenceThresholds};
    pub use crate::plugin::{InferencePlugin, InferenceState};
    pub use crate::{InferenceBackend, InferenceModel, InferenceModelConfig};
}
