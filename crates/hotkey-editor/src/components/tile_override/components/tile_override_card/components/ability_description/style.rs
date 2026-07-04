use crate::{classes, styling::TailwindClass, tw};

// The primary ubertip / tip text block for an ability or upgrade. A scrollable
// gold-edged panel on the sidebar; natural-height and smaller text on small screens.
// Each line is a `<p>` (pre-wrapped) so authored spacing survives.
const BASE: &[TailwindClass] = tw![
    "flex-1",
    "min-h-0",
    "overflow-y-auto",
    "flex",
    "flex-col",
    "gap-[0.4rem]",
    "px-4",
    "py-[0.85rem]",
    "bg-[rgba(8,18,35,0.35)]",
    "border-l-2",
    "border-warcraft-gold",
    "rounded-[0.25rem]",
    "text-[#c0c8da]",
    "text-[1.55rem]",
    "leading-[1.55]",
    "[&>p]:m-0",
    "[&>p]:whitespace-pre-wrap",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-none",
    "mobile:overflow-visible",
    "mobile:max-h-none",
    "mobile:text-[13px]",
    "mobile:leading-[1.35]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:flex-none",
    "tablet:overflow-visible",
    "tablet:max-h-none",
    "tablet:text-[13px]",
    "tablet:leading-[1.35]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
