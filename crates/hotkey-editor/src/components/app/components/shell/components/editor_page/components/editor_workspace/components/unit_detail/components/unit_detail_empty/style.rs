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
    "bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_70%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_70%,transparent)_100%)]",
    "shadow-[inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_4%,transparent)]",
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
