//! A library for window abstraction and event logic
//!
//! This library is used as an abstraction layer on top of
//! different window back-ends, such that libraries
//! can be written without more dependencies than required.
//!
//! An event loop can be created in different ways:
//!
//! ```ignore
//! // Move window by value (this prevents you from using the window elsewhere).
//! for e in Events::new(window) {
//!    ...
//! }
//!
//! // Use shared reference (this allows you to use the window elsewhere).
//! let window = RefCell::new(window);
//! for e in Events::new(&window) {
//!    ...
//! }
//!
//! // Use current window (the window must be set as current object).
//! for e in Events::new(current::UseCurrent::<Window>) {
//!    ...
//! }
//! 
//! // Specify usage.
//! let window = RefCell::new(window);
//! let usage = current::Use(&window);
//! for e in Events::new(usage) {
//!    ...
//! }
//! ```
//!
//! It is also designed to provide an extensible model for events,
//! such that window back-ends can add new kinds of events.
//! The new event can be created as trait and implemented for
//! all types that uses `GenericEvent`.
//! For examples, see the different events in this library.

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_docs)]
#![feature(default_type_params)]

extern crate time;
extern crate input;
extern crate serialize;
extern crate current;

pub use window::{
    Window,
    WindowSettings,
    NoWindow
};
pub use events::{
    Events,
    MaxFps,
    Ups,
};
pub use generic_event::{ assert_event_trait, GenericEvent };
pub use update::{ UpdateArgs, UpdateEvent };
pub use render::{ RenderArgs, RenderEvent };
pub use event::{ Event, Render, Update, Input };
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use text::TextEvent;
pub use resize::ResizeEvent;
pub use focus::FocusEvent;

pub mod ptr;
pub mod drag_controller;
pub mod window;

mod events;
mod generic_event;
mod update;
mod render;
mod event;
mod press;
mod release;
mod mouse;
mod text;
mod resize;
mod focus;
