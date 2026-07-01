use crate::classes;

// Three big hero slots in a single row on desktop; equal fractions capped to the
// dialog width on small viewports.
const BASE: &[&str] = &["grid", "grid-cols-[repeat(3,26rem)]", "gap-[1.5rem]"];
const MOBILE: &[&str] = &[
    "mobile:grid-cols-[repeat(3,minmax(0,1fr))]",
    "mobile:gap-[0.5rem]",
    "mobile:w-full",
    "mobile:max-w-[30rem]",
    "mobile:mx-auto",
];
const TABLET: &[&str] = &[
    "tablet:grid-cols-[repeat(3,minmax(0,1fr))]",
    "tablet:gap-[0.5rem]",
    "tablet:w-full",
    "tablet:max-w-[30rem]",
    "tablet:mx-auto",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
