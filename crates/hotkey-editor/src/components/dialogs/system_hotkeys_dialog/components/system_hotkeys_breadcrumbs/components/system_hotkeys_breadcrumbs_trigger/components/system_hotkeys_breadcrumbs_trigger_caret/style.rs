use crate::classes;

const BASE: &[&str] = &["data-[open=true]:rotate-180"];

const MOBILE: &[&str] = &[
    "mobile:flex-none",
    "mobile:ml-[0.6rem]",
    "mobile:text-[0.9em]",
    "mobile:leading-none",
    "mobile:[transition:transform_0.18s_ease]",
];

const TABLET: &[&str] = &[
    "tablet:flex-none",
    "tablet:ml-[0.6rem]",
    "tablet:text-[0.9em]",
    "tablet:leading-none",
    "tablet:[transition:transform_0.18s_ease]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
