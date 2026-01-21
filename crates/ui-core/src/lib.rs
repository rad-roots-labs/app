#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

mod id;
mod event;

pub use event::{
    radroots_app_ui_compose_event_handlers,
    radroots_app_ui_compose_event_handlers_unchecked,
};
pub use id::{RadrootsAppUiId, RadrootsAppUiIdSequence};
