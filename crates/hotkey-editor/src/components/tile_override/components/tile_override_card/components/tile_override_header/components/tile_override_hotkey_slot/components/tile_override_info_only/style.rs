use crate::{classes, styling::TailwindClass, tw};

// The muted italic note shown in place of a hotkey field for passive abilities.
const BASE: &[TailwindClass] = tw!["m-0", "text-[1.45rem]", "italic", "text-[#7b818d]"];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
