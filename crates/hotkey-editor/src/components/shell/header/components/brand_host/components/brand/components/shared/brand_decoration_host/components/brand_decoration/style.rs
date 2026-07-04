use crate::classes;

// Fills the host's inline size and keeps its own aspect ratio (`h-auto`), so it
// never distorts. The host owns the box; `100cqi` is the full width of the host's
// container-query context. No fixed length here — size is the host's decision.
const BASE: &[&str] = &[
    "block",
    "w-[100cqi]",
    "h-auto",
    "flex-none",
    "[filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
