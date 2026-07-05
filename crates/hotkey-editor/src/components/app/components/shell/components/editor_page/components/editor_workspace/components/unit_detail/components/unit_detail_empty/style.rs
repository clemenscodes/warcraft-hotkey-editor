use crate::{classes, styling::TailwindClass, tw};

// The unit-detail card in its empty state: the same bordered shell, centered muted
// italic message when no (or an invalid) unit is selected.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "min-w-0",
    "min-h-[16rem]",
    "p-8",
    "border",
    "border-warcraft-blue-deep",
    "rounded-[12px]",
    "bg-panel-dark-diag-70",
    "shadow-bevel-hl-3",
    "text-warcraft-text-faint",
    "text-[1rem]",
    "italic",
];

const MOBILE: &[TailwindClass] = tw!["mobile:p-[0.85rem]", "mobile:rounded-[6px]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
