use crate::classes;

const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(2,28rem)]",
    "auto-rows-[20rem]",
    "gap-[1.5rem]",
];

const MOBILE: &[&str] = &[
    "mobile:grid-cols-[repeat(2,minmax(0,1fr))]",
    "mobile:auto-rows-[minmax(86px,auto)]",
    "mobile:gap-[0.6rem]",
    "mobile:w-full",
    "mobile:max-w-[26rem]",
    "mobile:mx-auto",
];

const TABLET: &[&str] = &[
    "tablet:grid-cols-[repeat(2,minmax(0,1fr))]",
    "tablet:auto-rows-[minmax(86px,auto)]",
    "tablet:gap-[0.6rem]",
    "tablet:w-full",
    "tablet:max-w-[26rem]",
    "tablet:mx-auto",
];

const LAPTOP: &[&str] = &[
    "laptop:grid-cols-[repeat(2,230px)]",
    "laptop:auto-rows-[165px]",
    "laptop:gap-[12px]",
];

const DESKTOP: &[&str] = &[
    "desktop:grid-cols-[repeat(2,230px)]",
    "desktop:auto-rows-[165px]",
    "desktop:gap-[12px]",
];

const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
