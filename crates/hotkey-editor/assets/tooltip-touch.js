const LONG_PRESS_MS = 600;
const MOVE_CANCEL_THRESHOLD_PX = 10;
const AUTO_HIDE_MS = 3500;
const BANNER_ELEMENT_ID = "tooltip-mobile-banner";

let activePress = null;

function findTooltipAnchor(target) {
    if (!(target instanceof Element)) {
        return null;
    }
    const anchor = target.closest("[data-tooltip]");
    if (!anchor) {
        return null;
    }
    const tooltipText = anchor.getAttribute("data-tooltip");
    if (!tooltipText) {
        return null;
    }
    return anchor;
}

function ensureBannerElement() {
    const existingBanner = document.getElementById(BANNER_ELEMENT_ID);
    if (existingBanner) {
        return existingBanner;
    }
    const newBanner = document.createElement("div");
    newBanner.id = BANNER_ELEMENT_ID;
    document.body.appendChild(newBanner);
    return newBanner;
}

// `position: fixed` on the ::after pseudo-element doesn't escape ancestors
// that establish their own containing block — and many of our cells use
// `filter` (conflict glow, hover brightness, editing pulse) which does
// exactly that. Render the touch banner as a real `<div>` directly under
// `<body>` so it stays viewport-relative regardless of which cell triggered
// it, then keep the original anchor's `data-tooltip-active` flag in sync so
// the existing CSS hooks (e.g. red conflict tint while the tooltip is open)
// keep working.
const VIEWPORT_MARGIN_PX = 8;
const BANNER_GAP_PX = 8;

function positionBannerNearAnchor(banner, anchor) {
    const cellRect = anchor.getBoundingClientRect();
    const bannerRect = banner.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const viewportWidth = window.innerWidth;

    // Prefer placing the banner above the touched cell so the user's finger
    // doesn't cover the text. Fall back below if there isn't enough room.
    const spaceAbove = cellRect.top - VIEWPORT_MARGIN_PX;
    const fitsAbove = bannerRect.height + BANNER_GAP_PX <= spaceAbove;
    const top = fitsAbove
        ? cellRect.top - BANNER_GAP_PX - bannerRect.height
        : Math.min(
              cellRect.bottom + BANNER_GAP_PX,
              viewportHeight - VIEWPORT_MARGIN_PX - bannerRect.height,
          );

    // Horizontally centre on the cell, then clamp to the viewport so a cell
    // near the screen edge doesn't push the banner off-screen.
    const cellCenterX = (cellRect.left + cellRect.right) / 2;
    const halfBannerWidth = bannerRect.width / 2;
    const minLeft = VIEWPORT_MARGIN_PX;
    const maxLeft = viewportWidth - VIEWPORT_MARGIN_PX - bannerRect.width;
    const clampedLeft = Math.max(minLeft, Math.min(maxLeft, cellCenterX - halfBannerWidth));

    banner.style.top = `${top}px`;
    banner.style.left = `${clampedLeft}px`;
}

function showTooltip(anchor) {
    anchor.setAttribute("data-tooltip-active", "true");
    const tooltipText = anchor.getAttribute("data-tooltip");
    if (!tooltipText) {
        return;
    }
    const banner = ensureBannerElement();
    banner.textContent = tooltipText;
    banner.setAttribute("data-visible", "true");
    positionBannerNearAnchor(banner, anchor);
}

function hideTooltip(anchor) {
    anchor.removeAttribute("data-tooltip-active");
    const banner = document.getElementById(BANNER_ELEMENT_ID);
    if (banner) {
        banner.removeAttribute("data-visible");
    }
}

function cancelActivePress() {
    if (!activePress) {
        return;
    }
    if (activePress.revealTimerId !== null) {
        window.clearTimeout(activePress.revealTimerId);
    }
    if (activePress.autoHideTimerId !== null) {
        window.clearTimeout(activePress.autoHideTimerId);
    }
    hideTooltip(activePress.targetElement);
    activePress = null;
}

function onPointerDown(event) {
    if (event.pointerType !== "touch" && event.pointerType !== "pen") {
        return;
    }
    const anchor = findTooltipAnchor(event.target);
    if (!anchor) {
        return;
    }
    cancelActivePress();
    const pending = {
        targetElement: anchor,
        pointerId: event.pointerId,
        originX: event.clientX,
        originY: event.clientY,
        revealTimerId: null,
        autoHideTimerId: null,
    };
    activePress = pending;
    pending.revealTimerId = window.setTimeout(() => {
        if (activePress !== pending) {
            return;
        }
        pending.revealTimerId = null;
        // The same long-press gesture is also used by the inventory grid to
        // commit a drag (see crates/hotkey-editor/src/components/command_grid.rs,
        // LONG_PRESS_MS = 300). If a drag has already taken ownership of this
        // element, suppress the tooltip — the drag follower is the relevant UX.
        if (pending.targetElement.classList.contains("dragging-source")) {
            cancelActivePress();
            return;
        }
        showTooltip(pending.targetElement);
        pending.autoHideTimerId = window.setTimeout(() => {
            cancelActivePress();
        }, AUTO_HIDE_MS);
    }, LONG_PRESS_MS);
}

function onPointerMove(event) {
    if (!activePress || activePress.pointerId !== event.pointerId) {
        return;
    }
    const dx = event.clientX - activePress.originX;
    const dy = event.clientY - activePress.originY;
    const thresholdSquared = MOVE_CANCEL_THRESHOLD_PX * MOVE_CANCEL_THRESHOLD_PX;
    if (dx * dx + dy * dy > thresholdSquared) {
        cancelActivePress();
    }
}

function onPointerUpOrCancel(event) {
    if (!activePress || activePress.pointerId !== event.pointerId) {
        return;
    }
    // If the reveal timer hasn't fired yet, the gesture was a tap — dismiss
    // immediately. If it has fired, leave the auto-hide timer running so the
    // tooltip stays visible briefly after the finger lifts.
    if (activePress.revealTimerId !== null) {
        cancelActivePress();
    }
}

if (!window.__tooltipTouchInstalled) {
    window.__tooltipTouchInstalled = true;
    const listenerOptions = { capture: true, passive: true };
    document.addEventListener("pointerdown", onPointerDown, listenerOptions);
    document.addEventListener("pointermove", onPointerMove, listenerOptions);
    document.addEventListener("pointerup", onPointerUpOrCancel, listenerOptions);
    document.addEventListener("pointercancel", onPointerUpOrCancel, listenerOptions);
}
