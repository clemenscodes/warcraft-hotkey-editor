use crate::{classes, styling::TailwindClass, tw};

// The label column of the alt-state header; clamps its width so long names ellipsize
// rather than push the controls off the row.
const BASE: &[TailwindClass] = tw!["min-w-0"];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
