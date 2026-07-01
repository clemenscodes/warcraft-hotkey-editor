use crate::classes;

// The override panel header: name/id on the left, the hotkey slot pinned right, with
// a hairline divider under it. A fixed two-column grid on the mobile panel.
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[minmax(0,1fr)_auto]",
    "items-center",
    "gap-x-6",
    "pb-[0.85rem]",
    "border-b",
    "border-[#1f3d63]",
    "text-left",
];

const MOBILE: &[&str] = &[
    "mobile:grid-cols-[1fr_auto]",
    "mobile:gap-[8px]",
    "mobile:p-[0_0_10px_0]",
    "mobile:m-0",
    "mobile:flex-[0_0_auto]",
    "mobile:overflow-hidden",
    "mobile:w-full",
];

const TABLET: &[&str] = &[
    "tablet:grid-cols-[1fr_auto]",
    "tablet:gap-[8px]",
    "tablet:p-[0_0_10px_0]",
    "tablet:m-0",
    "tablet:flex-[0_0_auto]",
    "tablet:overflow-hidden",
    "tablet:w-full",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
