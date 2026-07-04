use crate::{classes, styling::TailwindClass, tw};

// The editor's grid slot: it establishes the query container the grid and its
// tiles size against, and carries the responsive width (four tiles plus gaps).
// Everything inside sizes in `cqi` off this width, so the whole grid is one
// scalable shape — a mini grid is the same `Grid` in a smaller container.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "flex-[1_1_0]",
    "min-w-0",
    "self-stretch",
    "[container-type:inline-size]",
    "max-w-[578px]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:max-w-[482px]"];
const TABLET: &[TailwindClass] = tw!["tablet:max-w-[530px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw!["desktop:max-w-[642px]"];
const QHD: &[TailwindClass] = tw!["qhd:max-w-[706px]"];
const UHD: &[TailwindClass] = tw!["uhd:max-w-[818px]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
