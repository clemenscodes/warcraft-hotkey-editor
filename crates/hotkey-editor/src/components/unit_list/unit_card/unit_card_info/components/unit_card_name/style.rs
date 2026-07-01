use crate::classes;

// The unit's display name. Ellipsized single line. On the sidebar it grows a touch
// on the widest screens; in the mobile carousel tile it shrinks to a fixed 11px.
const BASE: &[&str] = &[
    "text-[1.05rem]",
    "leading-[1.25]",
    "pb-[0.1rem]",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
    "min-w-0",
];
const MOBILE: &[&str] = &[
    "mobile:block",
    "mobile:w-full",
    "mobile:text-[11px]",
    "mobile:leading-[1.2]",
    "mobile:data-[selected=true]:text-white",
];
const TABLET: &[&str] = &[
    "tablet:block",
    "tablet:w-full",
    "tablet:text-[11px]",
    "tablet:leading-[1.2]",
    "tablet:data-[selected=true]:text-white",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &["desktop:text-[1.35rem]"];
const QHD: &[&str] = &["qhd:text-[1.35rem]"];
const UHD: &[&str] = &["uhd:text-[1.35rem]"];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
