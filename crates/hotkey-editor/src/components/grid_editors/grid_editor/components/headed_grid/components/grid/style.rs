use crate::classes;

// The generic grid shape: four equal columns of tile squares, filling whatever
// query container the extension gives it. The gap is in `cqi` so it scales with
// that container — the editor's is full width, a mini grid's is small, and the
// same shape renders at both sizes.
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(4,minmax(0,1fr))]",
    "gap-[1.04cqi]",
    "w-full",
    "overflow-visible",
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
