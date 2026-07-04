use crate::{classes, styling::TailwindClass, tw};

// Fills the host's inline size and keeps its own aspect ratio (`h-auto`), so it
// never distorts. The host owns the box; `100cqi` is the full width of the host's
// container-query context. No fixed length here — size is the host's decision.
const BASE: &[TailwindClass] = tw![
    "block",
    "w-[100cqi]",
    "h-auto",
    "flex-none",
    "[filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))]",
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
