// scripts/tooltip-touch.ts
var LONG_PRESS_MS = 600;
var MOVE_CANCEL_THRESHOLD_PX = 10;
var AUTO_HIDE_MS = 3500;
var BANNER_ELEMENT_ID = "tooltip-mobile-banner";
var activePress = null;
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
var VIEWPORT_MARGIN_PX = 8;
var BANNER_GAP_PX = 8;
function positionBannerNearAnchor(banner, anchor) {
  const cellRect = anchor.getBoundingClientRect();
  const bannerRect = banner.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;
  const spaceAbove = cellRect.top - VIEWPORT_MARGIN_PX;
  const spaceBelow = viewportHeight - cellRect.bottom - VIEWPORT_MARGIN_PX;
  const fitsAbove = bannerRect.height + BANNER_GAP_PX <= spaceAbove;
  const top = fitsAbove ? cellRect.top - BANNER_GAP_PX - bannerRect.height : Math.min(
    cellRect.bottom + BANNER_GAP_PX,
    viewportHeight - VIEWPORT_MARGIN_PX - bannerRect.height
  );
  const cellCenterX = (cellRect.left + cellRect.right) / 2;
  const halfBannerWidth = bannerRect.width / 2;
  const minLeft = VIEWPORT_MARGIN_PX;
  const maxLeft = viewportWidth - VIEWPORT_MARGIN_PX - bannerRect.width;
  const desiredLeft = cellCenterX - halfBannerWidth;
  const clampedLeft = Math.max(minLeft, Math.min(maxLeft, desiredLeft));
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
    autoHideTimerId: null
  };
  activePress = pending;
  pending.revealTimerId = window.setTimeout(() => {
    if (activePress !== pending) {
      return;
    }
    pending.revealTimerId = null;
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
