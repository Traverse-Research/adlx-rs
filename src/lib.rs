#![doc = include_str!("../README.md")]

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code
)]
pub mod ffi;

pub mod display;
pub mod display_free_sync;
pub mod display_list;
pub mod displays_services;
pub mod gpu;
pub mod gpu_list;
pub mod gpu_metrics;
pub mod helper;
pub mod interface;
pub mod list;
pub mod performance_monitoring_services;
pub mod result;
pub mod system;
pub mod three_d_settings_services;
pub mod three_d_wait_for_vertical_refresh;

pub use display::*;
pub use display_free_sync::*;
pub use display_list::*;
pub use displays_services::*;
pub use gpu::*;
pub use gpu_list::*;
pub use gpu_metrics::*;
pub use helper::*;
pub use interface::*;
pub use list::*;
pub use performance_monitoring_services::*;
pub use result::*;
pub use system::*;
pub use three_d_settings_services::*;
pub use three_d_wait_for_vertical_refresh::*;
