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
    "border-[#1f3d63]",
    "rounded-[12px]",
    "bg-[linear-gradient(135deg,rgba(13,31,61,0.7)_0%,rgba(6,12,31,0.7)_100%)]",
    "shadow-[inset_0_1px_0_rgba(255,255,255,0.04)]",
    "text-[#7b818d]",
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
