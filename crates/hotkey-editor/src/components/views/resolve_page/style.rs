use crate::classes;

/// `resolve-plan` is an e2e marker (`.resolve-plan[data-resolve-state="plan"]`).
const BASE: &[&str] = &[
    "resolve-plan",
    "flex",
    "flex-col",
    "flex-[1_1_0]",
    "min-h-0",
];
const MOBILE: &[&str] = &["mobile:flex-none"];
const TABLET: &[&str] = &["tablet:flex-none"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
