use crate::{classes, styling::TailwindClass, tw};

// A single pre-wrapped description line under the alt-state header.
const BASE: &[TailwindClass] = tw!["m-0", "whitespace-pre-wrap"];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
