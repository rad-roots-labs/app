#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

mod id;
mod event;
mod input;
mod preference;

pub use event::{
    radroots_app_ui_compose_event_handlers,
    radroots_app_ui_compose_event_handlers_unchecked,
};
pub use id::{RadrootsAppUiId, RadrootsAppUiIdSequence};
pub use input::{
    radroots_app_ui_input_modality_attach,
    radroots_app_ui_input_modality_get,
    radroots_app_ui_input_modality_set,
    RadrootsAppUiInputModality,
    RadrootsAppUiInputModalityError,
};
pub use preference::{
    radroots_app_ui_prefers_contrast_more,
    radroots_app_ui_prefers_reduced_motion,
    RadrootsAppUiPreferenceError,
    RadrootsAppUiPreferenceResult,
};
