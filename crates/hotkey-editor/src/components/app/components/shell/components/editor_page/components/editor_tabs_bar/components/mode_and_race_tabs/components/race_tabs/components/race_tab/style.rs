use crate::{classes, states, styling::TailwindClass, tw};
use warcraft_api::Race;

const BASE: &[TailwindClass] = tw![
    "group",
    "relative",
    "flex-1",
    "min-w-0",
    "p-0",
    "border",
    "border-[#3a4a6c]",
    "rounded-[8px]",
    "text-[#fff5d6]",
    "text-[1.5rem]",
    "uppercase",
    "tracking-[0.08em]",
    "text-center",
    "transition-[border-color,box-shadow,transform]",
    "duration-150",
    "overflow-hidden",
    "isolate",
    "flex",
    "items-end",
    "justify-center",
    "min-h-[clamp(4rem,7vw,7rem)]",
    "min-w-[clamp(4.5rem,8vw,9rem)]",
    "[text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000,0_0_8px_rgba(0,0,0,0.85)]",
    "before:content-['']",
    "before:absolute",
    "before:inset-0",
    "before:bg-contain",
    "before:bg-no-repeat",
    "before:bg-center",
    "before:brightness-150",
    "before:saturate-125",
    "before:z-0",
    "before:pointer-events-none",
    "before:block",
    "after:content-['']",
    "after:absolute",
    "after:inset-0",
    "after:bg-[linear-gradient(180deg,rgba(0,0,0,0)_0%,rgba(0,0,0,0)_45%,rgba(0,0,0,0.55)_75%,rgba(0,0,0,0.85)_100%)]",
    "after:z-[1]",
    "after:pointer-events-none",
    "after:block",
    "hover:text-white",
    "focus:outline-none",
    "data-[active=true]:text-white",
    "[body[data-kb-modality]_&]:focus:outline-none",
    "[body[data-kb-modality]_&]:focus:text-white",
    "[body[data-kb-modality]_&]:focus:border-white",
    "[body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_18px_rgba(255,255,255,0.55)]",
];
// Phone/tablet: a swipe-scannable banner-card strip — each tab a chunky fixed
// height (all five share the row) with tighter type and a slightly softer banner
// brightness than the desktop row.
const MOBILE: &[TailwindClass] = tw![
    "mobile:h-[clamp(112px,36vw,200px)]",
    "mobile:min-h-0",
    "mobile:text-[clamp(0.75rem,2.6vw,1rem)]",
    "mobile:before:brightness-[1.35]",
    "mobile:before:saturate-[1.2]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:h-[clamp(112px,36vw,200px)]",
    "tablet:min-h-0",
    "tablet:text-[clamp(0.75rem,2.6vw,1rem)]",
    "tablet:before:brightness-[1.35]",
    "tablet:before:saturate-[1.2]",
];
const LAPTOP: &[TailwindClass] = tw!["laptop:min-h-[clamp(3rem,6vh,5rem)]"];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const HUMAN: &[TailwindClass] = tw![
    "bg-[linear-gradient(180deg,#173266_0%,#050a1a_100%)]",
    "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-human.png')]",
    "hover:border-race-human",
    "hover:shadow-[0_0_12px_rgba(106,161,255,0.45)]",
    "data-[active=true]:border-race-human",
    "data-[active=true]:bg-[linear-gradient(180deg,#173266_0%,#0a1432_100%)]",
    "data-[active=true]:shadow-[0_0_22px_rgba(106,161,255,0.45),inset_0_0_24px_rgba(255,255,255,0.04)]",
];
const ORC: &[TailwindClass] = tw![
    "bg-[linear-gradient(180deg,#5a1212_0%,#050a1a_100%)]",
    "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-orc.png')]",
    "hover:border-race-orc",
    "hover:shadow-[0_0_12px_rgba(255,122,122,0.45)]",
    "data-[active=true]:border-race-orc",
    "data-[active=true]:bg-[linear-gradient(180deg,#5a1212_0%,#0a1432_100%)]",
    "data-[active=true]:shadow-[0_0_22px_rgba(255,122,122,0.45),inset_0_0_24px_rgba(255,255,255,0.04)]",
];
const NIGHTELF: &[TailwindClass] = tw![
    "bg-[linear-gradient(180deg,#0c4348_0%,#050a1a_100%)]",
    "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-nightelf.png')]",
    "hover:border-race-nightelf",
    "hover:shadow-[0_0_12px_rgba(95,218,218,0.45)]",
    "data-[active=true]:border-race-nightelf",
    "data-[active=true]:bg-[linear-gradient(180deg,#0c4348_0%,#0a1432_100%)]",
    "data-[active=true]:shadow-[0_0_22px_rgba(95,218,218,0.45),inset_0_0_24px_rgba(255,255,255,0.04)]",
];
const UNDEAD: &[TailwindClass] = tw![
    "bg-[linear-gradient(180deg,#321650_0%,#050a1a_100%)]",
    "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-undead.png')]",
    "hover:border-race-undead",
    "hover:shadow-[0_0_12px_rgba(199,155,255,0.45)]",
    "data-[active=true]:border-race-undead",
    "data-[active=true]:bg-[linear-gradient(180deg,#321650_0%,#0a1432_100%)]",
    "data-[active=true]:shadow-[0_0_22px_rgba(199,155,255,0.45),inset_0_0_24px_rgba(255,255,255,0.04)]",
];
const NEUTRAL: &[TailwindClass] = tw![
    "bg-[linear-gradient(180deg,#4a3d14_0%,#050a1a_100%)]",
    "before:bg-[url('/warcraft-hotkey-editor/webui/common/dark-banner-random.png')]",
    "hover:border-warcraft-gold",
    "hover:shadow-[0_0_12px_rgba(255,206,99,0.45)]",
    "data-[active=true]:border-warcraft-gold",
    "data-[active=true]:bg-[linear-gradient(180deg,#4a3d14_0%,#0a1432_100%)]",
    "data-[active=true]:shadow-[0_0_22px_rgba(255,206,99,0.45),inset_0_0_24px_rgba(255,255,255,0.04)]",
];
states! {
    Race, Human => HUMAN, Orc => ORC, Nightelf => NIGHTELF, Undead => UNDEAD, Neutral => NEUTRAL
}
