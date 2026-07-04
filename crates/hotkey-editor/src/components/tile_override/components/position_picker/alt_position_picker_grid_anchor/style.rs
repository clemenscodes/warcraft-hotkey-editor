use crate::{classes, styling::TailwindClass, tw};

// Centers the embedded command grid inside a position-picker dialog and restyles its
// tiles for the picker's single-button drag: non-draggable tiles dim out,
// drop-target and drag-over cells glow gold. All descendant overrides are `!` so they
// win over the grid editor's own tile styling.
const BASE: &[TailwindClass] = tw![
    "flex",
    "justify-center",
    "w-full",
    "[&_.grid-section]:[container-type:normal]",
    "[&_.grid-section]:w-max",
    "[&_.grid]:[--tile-size:8rem]",
    "[&_.grid]:grid-cols-[repeat(4,var(--tile-size))]",
    "[&_.grid]:[grid-auto-rows:var(--tile-size)]",
    "[&_.filled-tile]:w-[var(--tile-size)]",
    "[&_.filled-tile]:h-[var(--tile-size)]",
    "[&_.empty-tile]:w-[var(--tile-size)]",
    "[&_.empty-tile]:h-[var(--tile-size)]",
    "[&_.command-tile-wrapper]:w-[var(--tile-size)]",
    "[&_.command-tile-wrapper]:h-[var(--tile-size)]",
    "[&_.filled-tile[data-draggable=false]]:cursor-default!",
    "[&_.filled-tile[data-draggable=false]]:opacity-[0.32]!",
    "[&_.filled-tile[data-draggable=false]]:[filter:saturate(0.35)_brightness(0.85)]!",
    "[&_.filled-tile[data-draggable=false]]:border-[#233a5a]!",
    "[&_.filled-tile[data-draggable=false]]:[box-shadow:inset_0_1px_0_rgba(255,255,255,0.04),0_1px_2px_rgba(0,0,0,0.5)]!",
    "[&_.filled-tile[data-draggable=false]]:[transform:none]!",
    "[&_.empty-tile[data-draggable=false]]:[border:2px_dashed_rgba(255,206,99,0.45)]!",
    "[&_.empty-tile[data-draggable=false]]:[background:rgba(255,206,99,0.04)]!",
    "[&_.empty-tile[data-draggable=false]]:hover:border-[rgba(255,206,99,0.75)]!",
    "[&_.empty-tile[data-draggable=false]]:hover:[background:rgba(255,206,99,0.1)]!",
    "[&_.empty-tile[data-drop-target=true]]:[border:2px_solid_rgba(255,206,99,0.75)]!",
    "[&_.empty-tile[data-drop-target=true]]:[background:rgba(255,206,99,0.14)]!",
    "[&_.empty-tile[data-drop-target=true]]:cursor-pointer!",
    "[&_.empty-tile[data-drop-target=true]]:[box-shadow:0_0_12px_rgba(255,206,99,0.25)]!",
    "[&_.empty-tile[data-drag-over=true]]:[border:2px_solid_#ffce63]!",
    "[&_.empty-tile[data-drag-over=true]]:[background:linear-gradient(135deg,rgba(255,206,99,0.32)_0%,rgba(255,206,99,0.18)_100%),rgba(20,35,60,0.95)]!",
    "[&_.empty-tile[data-drag-over=true]]:[box-shadow:inset_0_0_0_1px_rgba(255,224,150,0.65),0_0_28px_rgba(255,206,99,0.85)]!",
    "[&_.filled-tile[data-draggable=true]]:cursor-grab!",
    "[&_.filled-tile[data-draggable=true]]:border-[#ffce63]!",
    "[&_.filled-tile[data-draggable=true]]:[transition:none]!",
    "[&_.filled-tile[data-draggable=true]]:active:cursor-grabbing!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[border:2px_dashed_rgba(255,206,99,0.45)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[background:rgba(255,206,99,0.04)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[box-shadow:none]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[transform:none]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[filter:none]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[border:3px_solid_#ffce63]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[background:linear-gradient(135deg,rgba(255,206,99,0.32)_0%,rgba(255,206,99,0.18)_100%),rgba(20,35,60,0.95)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[box-shadow:inset_0_0_0_1px_rgba(255,224,150,0.65),0_0_28px_rgba(255,206,99,0.85)]!",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
