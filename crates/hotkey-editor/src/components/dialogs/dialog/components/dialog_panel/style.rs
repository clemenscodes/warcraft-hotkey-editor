use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "w-dialog-w",
    "h-dialog-h",
    "p-0",
    "gap-0",
    "overflow-hidden",
    "rounded-xl",
    "surface-panel",
];
const MOBILE: &[&str] = &[
    "mobile:w-screen",
    "mobile:h-dvh",
    "mobile:max-w-screen",
    "mobile:max-h-dvh",
    "mobile:rounded-none",
    "mobile:border-x-0",
];
const TABLET: &[&str] = &[
    "tablet:w-dialog-w-roomy",
    "tablet:h-dialog-h-roomy",
    "tablet:max-w-dialog-w-roomy",
    "tablet:max-h-dialog-h-roomy",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
