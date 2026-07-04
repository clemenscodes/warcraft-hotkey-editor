use crate::{classes, styling::TailwindClass, tw};

// The interactive wrapper coincides with the base tile it wraps: same square, same
// query container, so the hotkey badge's `cqi` sizing is unchanged and the drag
// overlays sit exactly over the tile. All sizing is in `cqi`, so it scales with the
// grid just like the base tile.
const BASE: &[TailwindClass] = tw![
    "relative",
    "w-full",
    "aspect-square",
    "[container-type:inline-size]",
    "rounded-[1.04cqi]",
    "touch-pan-y",
    "cursor-grab",
    "outline-none",
    "kb-focus:[box-shadow:0_0_0_0.52cqi_#ffce63,0_0_3.1cqi_rgba(255,206,99,0.55)]",
    "data-[drag-over=true]:[box-shadow:0_0_0_0.35cqi_#ffce63]",
    "[body:has([data-dragging-source=true])_&]:cursor-grabbing",
    "data-[dragging-source=true]:[&>*]:invisible",
    "data-[dragging-source=true]:border-[0.35cqi]",
    "data-[dragging-source=true]:border-dashed",
    "data-[dragging-source=true]:border-[#4a7090]",
    "data-[dragging-source=true]:[background:linear-gradient(135deg,rgba(15,30,55,0.85)_0%,rgba(8,14,30,0.85)_100%)]",
    "data-[dragging-source=true]:[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]",
    "data-[dragging-source=true]:data-[drag-over=true]:[box-shadow:0_0_0_0.35cqi_#ffce63]",
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
