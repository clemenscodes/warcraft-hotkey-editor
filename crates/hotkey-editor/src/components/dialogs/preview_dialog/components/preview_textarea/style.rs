use crate::classes;

const BASE: &[&str] = &[
    "w-full",
    "flex-1",
    "min-h-preview",
    "px-8",
    "py-6",
    "rounded-md",
    "border",
    "border-warcraft-blue",
    "bg-warcraft-bg-deep",
    "text-warcraft-text-primary",
    "font-mono",
    "text-preview",
    "whitespace-pre",
    "overflow-auto",
    "resize-y",
    "scrollbar-gold-axes",
    "focus:outline-none",
    "focus:border-warcraft-gold",
    "focus:shadow-gold-focus",
];
const MOBILE: &[&str] = &["mobile:text-preview-sm"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
