use crate::{classes, styling::TailwindClass, tw};

// A dialog host is a data seam, not a box: it owns no layout of its own. Its overlay
// child positions itself fixed, so the host root is `contents` — a layout-neutral
// grouping wrapper carrying only the identity class, adding no box to the toolbar row.
const BASE: &[TailwindClass] = tw!["contents"];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
