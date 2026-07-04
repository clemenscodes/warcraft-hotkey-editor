use crate::{classes, styling::TailwindClass, tw};

// The inline file-button row is shown from laptop up (the burger replaces it below).
// Its height is the box every toolbar button fills, and `h-full` takes it straight from
// the toolbar's shared row height — so the whole row rescales with the bar and matches the
// collisions and layout buttons, with no fixed size of its own. `gap-1` is the same fixed
// inter-button gap the toolbar uses; the tight gap keeps the full ten-button row inside its
// grid track.
// The row stays mounted at every width so each button can carry the dialog it opens (a
// fixed overlay that must render on mobile too, where the burger flips the shared open
// signal). Below laptop every button's own slot is `hidden`, so the row collapses to
// nothing visible while its dialogs stay live; the burger sits beside it as the compact
// trigger. The row's flex layout is therefore always-on.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-1",
    "min-w-0",
    "h-full",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
