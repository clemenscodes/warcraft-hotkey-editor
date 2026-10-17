//! The spring pager: a strip of cards a finger drags one to one and that springs
//! onto the next card when let go. Shared by every pager on the touch bands, so
//! they all move the same way. Each pager hands it the selector of its own card
//! hosts and the signals it settles through.

mod card_resize_watch;
mod last_card_watch;
mod pager_dom;
mod pager_motion;
mod pager_window;
mod passive_input_listeners;
mod spring_curve;

pub(crate) use card_resize_watch::CardResizeWatch;
pub(crate) use last_card_watch::LastCardWatch;
pub(crate) use pager_dom::PagerDom;
pub(crate) use pager_motion::PagerMotion;
pub(crate) use pager_window::{PagerAnchor, PagerWindow, RosterSlot};
pub(crate) use passive_input_listeners::PassiveInputListeners;
