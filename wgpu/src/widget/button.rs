//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
//!
//! [`Button`]: type.Button.html
//! [`State`]: struct.State.html
use crate::Renderer;

pub use iced_native::button::State;
pub use iced_style::button::{Style, StyleSheet};

/// A widget that produces a message when clicked.
///
/// This is an alias of an `iced_native` button with an `iced_wgpu::Renderer`.
pub type Button<'a, Message> = iced_native::Button<'a, Message, Renderer>;
