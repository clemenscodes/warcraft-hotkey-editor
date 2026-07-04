use crate::{classes, styling::TailwindClass, tw};

// The painter's box and the hotkey badge's query container: the square tile shape with
// rounded corners, marked a container so the badge (a descendant) sizes in `cqi` against
// this box whether or not a Host wraps the painter — the templates preview renders the
// painter alone, so the container must live here, not on the Host. All interaction visuals
// (the drag-over ring, the dragging-source ghost, the focus ring) live on the
// `GridEditorTile` Host, whose own `cqi` resolves against the outer grid; none of them
// belong to the painter.
const BASE: &[TailwindClass] = tw![
    "relative",
    "w-full",
    "aspect-square",
    "[container-type:inline-size]",
    "rounded-[1.04cqi]",
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
