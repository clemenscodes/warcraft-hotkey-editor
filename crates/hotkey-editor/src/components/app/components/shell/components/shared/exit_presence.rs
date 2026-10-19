use dioxus::prelude::*;
use std::time::Duration;

/// How long a closed popup stays mounted, matching `--animate-dialog-out`.
const EXIT_DURATION: Duration = Duration::from_millis(100);

/// Where a popup is in its life on screen. A closed popup stays mounted while
/// its exit animation plays, so it leaves the screen instead of vanishing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub(crate) enum Presence {
    #[default]
    Hidden,
    Shown,
    Leaving,
}

impl Presence {
    pub(crate) fn is_mounted(self) -> bool {
        self != Self::Hidden
    }

    pub(crate) fn is_shown(self) -> bool {
        self == Self::Shown
    }

    pub(crate) fn is_leaving(self) -> bool {
        self == Self::Leaving
    }
}

/// Opens and closes a popup, holding it mounted for its exit animation.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct ExitPresence {
    presence: Signal<Presence>,
    // Bumped on every open and close, so a stale exit timer cannot hide a popup
    // that was opened again while it was still leaving.
    generation: Signal<u32>,
}

impl ExitPresence {
    pub(crate) fn presence(&self) -> Presence {
        *self.presence.read()
    }

    pub(crate) fn open(&mut self) {
        let next_generation = self.generation.peek().wrapping_add(1);
        self.generation.set(next_generation);
        self.presence.set(Presence::Shown);
    }

    pub(crate) fn close(&mut self) {
        if !self.presence.peek().is_shown() {
            return;
        }
        let next_generation = self.generation.peek().wrapping_add(1);
        self.generation.set(next_generation);
        self.presence.set(Presence::Leaving);
        let mut presence = self.presence;
        let generation = self.generation;
        spawn(async move {
            gloo_timers::future::sleep(EXIT_DURATION).await;
            if *generation.peek() == next_generation {
                presence.set(Presence::Hidden);
            }
        });
    }

    pub(crate) fn toggle(&mut self) {
        if self.presence.peek().is_shown() {
            self.close();
        } else {
            self.open();
        }
    }
}

pub(crate) fn use_exit_presence() -> ExitPresence {
    let presence = use_signal(Presence::default);
    let generation = use_signal::<u32>(|| 0);
    ExitPresence {
        presence,
        generation,
    }
}
