use crate::classes;

// The editor's grid slot: it establishes the query container the grid and its
// tiles size against, and carries the responsive width (four tiles plus gaps).
// Everything inside sizes in `cqi` off this width, so the whole grid is one
// scalable shape — a mini grid is the same `Grid` in a smaller container.
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "flex-[1_1_0]",
    "min-w-0",
    "self-stretch",
    "[container-type:inline-size]",
    "max-w-[578px]",
];

const MOBILE: &[&str] = &["mobile:max-w-[482px]"];
const TABLET: &[&str] = &["tablet:max-w-[530px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &["desktop:max-w-[642px]"];
const QHD: &[&str] = &["qhd:max-w-[706px]"];
const UHD: &[&str] = &["uhd:max-w-[818px]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
