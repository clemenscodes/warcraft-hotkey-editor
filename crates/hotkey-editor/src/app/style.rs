use super::state::AppLayout;
use crate::{classes, states};

// The app shell centres the workbench in a single column with generous padding that
// tightens toward phones. It also owns the app-wide foundational resets that used to
// live as global rules in tailwind.input.css: they are `.app`-scoped, so they are
// inline descendant utilities on this root, not global CSS. Old pixel breakpoints
// fold into the named bands (the former `<1099px` foundation → mobile+tablet, the
// `<767px` input floor → mobile).
const BASE: &[&str] = &[
    "mx-auto",
    "flex",
    "flex-col",
    "px-4",
    "min-h-[100dvh]",
    "min-w-0",
    "max-w-[100vw]",
    "overflow-x-clip",
    "gap-8",
    // App-wide typography + the fixed full-viewport background, formerly global
    // html/body rules — inline on the root now. The background rides a fixed
    // `::before` layer so it still fills the whole viewport even though the shell
    // itself is a centred column. `cursor-pointer` on descendant buttons replaces
    // the old global `:where(button)` rule.
    "font-display",
    "text-warcraft-text-primary",
    "[-webkit-text-size-adjust:100%]",
    "[text-size-adjust:100%]",
    "[&_button]:cursor-pointer",
    "before:content-['']",
    "before:fixed",
    "before:inset-0",
    "before:-z-10",
    "before:[background-color:#050a1a]",
    "before:[background-image:radial-gradient(ellipse_90%_60%_at_50%_0%,#18365b_0%,transparent_60%),linear-gradient(180deg,#0a1a35_0%,#050a1a_100%)]",
    "before:bg-no-repeat",
    "before:[background-attachment:fixed]",
    "before:[background-size:100%_100%]",
    // Neutralise sticky :hover on no-hover (touch) devices, where a tap leaves the
    // button in :hover. Formerly a global `@media(hover:none)` rule on `.app`.
    "[@media(hover:none)]:[&_button:hover]:[background:inherit]",
    "[@media(hover:none)]:[&_button:hover]:[color:inherit]",
    "[@media(hover:none)]:[&_button:hover]:[border-color:inherit]",
    "[@media(hover:none)]:[&_button:hover]:[box-shadow:none]",
    "[@media(hover:none)]:[&_button:hover]:[text-shadow:none]",
    "[@media(hover:none)]:[&_button:hover]:[transform:none]",
    "[@media(hover:none)]:[&_a:hover]:[background:inherit]",
    "[@media(hover:none)]:[&_a:hover]:[color:inherit]",
    "[@media(hover:none)]:[&_a:hover]:[border-color:inherit]",
    "[@media(hover:none)]:[&_a:hover]:[box-shadow:none]",
    "[@media(hover:none)]:[&_a:hover]:[text-shadow:none]",
    "[@media(hover:none)]:[&_a:hover]:[transform:none]",
    // The gold scrollbar treatment, formerly a global `*` rule. `scrollbar-width`
    // /`scrollbar-color` inherit down to every scrollable descendant; the webkit
    // pseudo-elements are matched on `.app`'s descendants.
    "[scrollbar-width:thin]",
    "[scrollbar-color:#ffce6373_transparent]",
    "[&_*::-webkit-scrollbar]:w-[8px]",
    "[&_*::-webkit-scrollbar]:h-[8px]",
    "[&_*::-webkit-scrollbar-track]:rounded-[4px]",
    "[&_*::-webkit-scrollbar-track]:[background:#08122373]",
    "[&_*::-webkit-scrollbar-thumb]:rounded-[4px]",
    "[&_*::-webkit-scrollbar-thumb]:[background:#ffce6373]",
    "[&_*::-webkit-scrollbar-thumb]:hover:[background:#ffce63]",
    "[&_*::-webkit-scrollbar-corner]:[background:#08122373]",
];
// Phone/tablet: no top padding, safe-area-aware side/bottom padding (so notches
// never clip the shell), a content-driven height above the dvh floor, the tighter
// section gap, and the `<1099px` foundational resets — force every descendant to
// shrink below its min-content and cap images so nothing pushes past the viewport.
// The `<767px` iOS-zoom input floor (`max(1rem,16px)`) is phone-only.
const MOBILE: &[&str] = &[
    "mobile:pl-[max(0.75rem,env(safe-area-inset-left))]",
    "mobile:pr-[max(0.75rem,env(safe-area-inset-right))]",
    "mobile:pb-[max(1rem,env(safe-area-inset-bottom))]",
    "mobile:h-auto",
    "mobile:overflow-y-visible",
    "mobile:gap-[16px]",
    "mobile:[&_*]:min-w-0",
    "mobile:[&_img]:max-w-full",
    "mobile:[&_img]:h-auto",
    "mobile:[&_svg]:max-w-full",
    "mobile:[&_svg]:h-auto",
    "mobile:[&_input]:text-[max(1rem,16px)]",
    "mobile:[&_textarea]:text-[max(1rem,16px)]",
    "mobile:[&_select]:text-[max(1rem,16px)]",
    "mobile:[&_button]:text-[max(1rem,16px)]",
    "mobile:[&_button]:[-webkit-tap-highlight-color:transparent]",
    "mobile:[&_button]:touch-manipulation",
    "mobile:[&_a]:[-webkit-tap-highlight-color:transparent]",
    "mobile:[&_a]:touch-manipulation",
    "mobile:[&_[role=button]]:[-webkit-tap-highlight-color:transparent]",
    "mobile:[&_[role=button]]:touch-manipulation",
    "mobile:[&_input]:touch-manipulation",
];
const TABLET: &[&str] = &[
    "tablet:pl-[max(0.75rem,env(safe-area-inset-left))]",
    "tablet:pr-[max(0.75rem,env(safe-area-inset-right))]",
    "tablet:pb-[max(1rem,env(safe-area-inset-bottom))]",
    "tablet:h-auto",
    "tablet:overflow-y-visible",
    "tablet:gap-[16px]",
    "tablet:[&_*]:min-w-0",
    "tablet:[&_img]:max-w-full",
    "tablet:[&_img]:h-auto",
    "tablet:[&_svg]:max-w-full",
    "tablet:[&_svg]:h-auto",
    "tablet:[&_button]:[-webkit-tap-highlight-color:transparent]",
    "tablet:[&_button]:touch-manipulation",
    "tablet:[&_a]:[-webkit-tap-highlight-color:transparent]",
    "tablet:[&_a]:touch-manipulation",
    "tablet:[&_[role=button]]:[-webkit-tap-highlight-color:transparent]",
    "tablet:[&_[role=button]]:touch-manipulation",
    "tablet:[&_input]:touch-manipulation",
];
const LAPTOP: &[&str] = &["laptop:gap-4"];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

// The collisions view is one full-bleed page and drops the inter-section gap; every
// other view keeps the standard responsive gaps from the bands above. `gap-0!` wins
// over those band gaps regardless of the utility order Tailwind emits.
const STANDARD: &[&str] = &[];
const COLLISIONS: &[&str] = &["gap-0!"];
states! {
    AppLayout, Standard => STANDARD, Collisions => COLLISIONS
}
