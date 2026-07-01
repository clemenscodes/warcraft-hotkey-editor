use crate::classes;

// The object id under the name, in a monospace face; smaller on the mobile panel.
const BASE: &[&str] = &["font-mono", "text-[1.4rem]", "text-[#7b818d]"];

const MOBILE: &[&str] = &[
    "mobile:m-0",
    "mobile:text-[12px]",
    "mobile:leading-[1.2]",
    "mobile:overflow-hidden",
    "mobile:whitespace-nowrap",
    "mobile:text-ellipsis",
];

const TABLET: &[&str] = &[
    "tablet:m-0",
    "tablet:text-[12px]",
    "tablet:leading-[1.2]",
    "tablet:overflow-hidden",
    "tablet:whitespace-nowrap",
    "tablet:text-ellipsis",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
