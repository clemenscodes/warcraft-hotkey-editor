use crate::classes;

/// The two-pane layout (sidebar column + fluid detail), self-contained here rather
/// than borrowed from the global `.main-content`. `collisions-page` is an e2e hook.
const BASE: &[&str] = &[
    "collisions-page",
    "grid",
    "grid-cols-[var(--sidebar-column-width)_minmax(0,1fr)]",
    "gap-10",
    "items-stretch",
    "mt-6",
    "flex-[1_1_0]",
    "min-h-0",
];
const MOBILE: &[&str] = &[
    "mobile:grid-cols-[1fr]",
    "mobile:flex-none",
    "mobile:min-h-[auto]",
];
const TABLET: &[&str] = &["tablet:grid-cols-[18rem_minmax(0,1fr)]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
