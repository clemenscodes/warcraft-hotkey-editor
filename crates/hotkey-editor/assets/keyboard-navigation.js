// Spatial (2D) keyboard navigation for desktop-width viewports: HJKL / arrow keys
// move focus to the nearest focusable neighbour in that direction. This legitimately
// needs the DOM — "nearest neighbour" is a geometry question over the rendered layout,
// not something application state can answer — so it measures element rects here.
//
// Focus *modality* (keyboard vs pointer focus rings) is now the platform's own
// :focus-visible, and post-activation focus hand-off is driven by application state
// through the Rust FocusCoordinator. Neither lives in this file anymore.

const NAV_SELECTORS = [
    ".upload-button .toolbar-button-surface",
    ".toggle-button",
    ".race-tab",
    'input[type="search"]',
    ".unit-card",
    ".unit-category-heading",
    ".grid-editor-tile",
    ".override-key",
    ".tile-override-tier-button",
    ".layout-tile",
    ".key-capture",
    ".inventory-filled-slot",
    ".slot-button",
    ".dialog-close",
];

const NAV_SELECTOR = NAV_SELECTORS.join(", ");
const PERPENDICULAR_WEIGHT = 2;
const TOLERANCE_PIXELS = 2;
const INSTALL_VERSION = 9;
const MIN_NAV_VIEWPORT_WIDTH = 1100;

function isNavigationViewport() {
    return document.documentElement.clientWidth >= MIN_NAV_VIEWPORT_WIDTH;
}

function directionFor(event) {
    const key = event.key;
    if (key === "h" || key === "H" || key === "ArrowLeft") return "left";
    if (key === "l" || key === "L" || key === "ArrowRight") return "right";
    if (key === "k" || key === "K" || key === "ArrowUp") return "up";
    if (key === "j" || key === "J" || key === "ArrowDown") return "down";
    return null;
}

function moveSpatially(direction) {
    if (!isNavigationViewport()) return;
    const active = document.activeElement;
    if (!active || !(active instanceof HTMLElement)) return;
    const activeRect = active.getBoundingClientRect();
    if (activeRect.width <= 0 && activeRect.height <= 0) return;

    const activeCenterX = (activeRect.left + activeRect.right) / 2;
    const activeCenterY = (activeRect.top + activeRect.bottom) / 2;

    const candidates = document.querySelectorAll(NAV_SELECTOR);
    let bestPenalty = Number.POSITIVE_INFINITY;
    let bestTarget = null;

    for (const candidate of candidates) {
        if (candidate === active) continue;
        if (candidate.tabIndex < 0) continue;

        const candidateRect = candidate.getBoundingClientRect();
        if (candidateRect.width <= 0 || candidateRect.height <= 0) continue;

        const candidateCenterX = (candidateRect.left + candidateRect.right) / 2;
        const candidateCenterY = (candidateRect.top + candidateRect.bottom) / 2;

        let primary;
        let perpendicular;
        switch (direction) {
            case "right":
                primary = candidateRect.left - activeRect.right;
                if (primary < -TOLERANCE_PIXELS) continue;
                perpendicular = Math.abs(candidateCenterY - activeCenterY);
                break;
            case "left":
                primary = activeRect.left - candidateRect.right;
                if (primary < -TOLERANCE_PIXELS) continue;
                perpendicular = Math.abs(candidateCenterY - activeCenterY);
                break;
            case "down":
                primary = candidateRect.top - activeRect.bottom;
                if (primary < -TOLERANCE_PIXELS) continue;
                perpendicular = Math.abs(candidateCenterX - activeCenterX);
                break;
            case "up":
                primary = activeRect.top - candidateRect.bottom;
                if (primary < -TOLERANCE_PIXELS) continue;
                perpendicular = Math.abs(candidateCenterX - activeCenterX);
                break;
        }

        const penalty = Math.max(primary, 0) + perpendicular * PERPENDICULAR_WEIGHT;
        if (penalty < bestPenalty) {
            bestPenalty = penalty;
            bestTarget = candidate;
        }
    }

    if (bestTarget) bestTarget.focus();
}

function installSpatialNavigation() {
    document.addEventListener(
        "keydown",
        (event) => {
            if (!isNavigationViewport()) return;
            if (event.ctrlKey || event.metaKey || event.altKey) return;
            // While a hotkey picker is open, H/J/K/L and the arrows are hotkey
            // candidates the picker must receive — don't hijack them for spatial
            // navigation. Either picker board being present means one is open.
            if (document.querySelector(".key-picker-board, .system-key-picker-board")) return;
            const target = event.target;
            if (target instanceof HTMLElement) {
                const tag = target.tagName;
                if (tag === "INPUT" || tag === "TEXTAREA") return;
            }
            const direction = directionFor(event);
            if (!direction) return;
            event.preventDefault();
            event.stopPropagation();
            moveSpatially(direction);
        },
        true,
    );
}

if (window.__kbNavigationVersion !== INSTALL_VERSION) {
    window.__kbNavigationVersion = INSTALL_VERSION;
    installSpatialNavigation();
}
