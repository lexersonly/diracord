//! Built-in function files, one folder per category, one file per function.
//!
//! To add a function: create `src/builtins/<category>/<name>.rs` exporting
//! `pub const SPEC: FunctionSpec`, declare it in the category's `mod.rs`, and
//! list it in [`all`].

pub mod common;
pub mod data;
pub mod lists;
pub mod lookup;
pub mod math;
pub mod messages;
pub mod moderation;
pub mod roles;
pub mod text;
pub mod utility;

use crate::FunctionSpec;

/// Every built-in function, in documentation order.
pub fn all() -> &'static [&'static FunctionSpec] {
    &[
        &messages::reply::SPEC,
        &messages::send::SPEC,
        &messages::dm::SPEC,
        &messages::react::SPEC,
        &messages::delete_message::SPEC,
        &moderation::ban::SPEC,
        &moderation::unban::SPEC,
        &moderation::kick::SPEC,
        &moderation::timeout::SPEC,
        &roles::add_role::SPEC,
        &roles::remove_role::SPEC,
        &roles::has_role::SPEC,
        &lookup::channel::SPEC,
        &lookup::member::SPEC,
        &lookup::role::SPEC,
        &data::get::SPEC,
        &data::set::SPEC,
        &data::delete::SPEC,
        &text::upper::SPEC,
        &text::lower::SPEC,
        &text::trim::SPEC,
        &text::len::SPEC,
        &text::replace::SPEC,
        &text::split::SPEC,
        &text::starts_with::SPEC,
        &text::ends_with::SPEC,
        &text::contains::SPEC,
        &math::random::SPEC,
        &math::round::SPEC,
        &math::floor::SPEC,
        &math::ceil::SPEC,
        &math::abs::SPEC,
        &math::min::SPEC,
        &math::max::SPEC,
        &lists::push::SPEC,
        &lists::join::SPEC,
        &lists::first::SPEC,
        &lists::last::SPEC,
        &lists::range::SPEC,
        &lists::reverse::SPEC,
        &utility::type_of::SPEC,
        &utility::to_number::SPEC,
        &utility::to_text::SPEC,
        &utility::now::SPEC,
    ]
}
