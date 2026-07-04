use crate::{classes, styling::TailwindClass, tw};

// The generic grid shape: four equal columns of tile squares, filling whatever
// query container the extension gives it. The gap is in `cqi` so it scales with
// that container — the editor's is full width, a mini grid's is small, and the
// same shape renders at both sizes.
const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[repeat(4,minmax(0,1fr))]",
    "gap-[1.04cqi]",
    "w-full",
    "overflow-visible",
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
