use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-center",
    "leading-none",
    "w-[2.2rem]",
    "h-[2.2rem]",
    "[&_svg]:block",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
];
const MOBILE: &[&str] = &["mobile:w-[1.4rem]", "mobile:h-[1.4rem]"];
const TABLET: &[&str] = &["tablet:w-[1.4rem]", "tablet:h-[1.4rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
