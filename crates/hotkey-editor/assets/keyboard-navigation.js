// scripts/keyboard-navigation.ts
var NAV_SELECTORS = [
  ".upload-button",
  ".mode-toggle-button",
  ".race-tab",
  ".unit-card",
  ".unit-category-heading",
  ".grid-tile.has-ability",
  ".override-key-cell",
  ".tile-override-tier-button",
  ".layout-cell",
  ".system-key-cell",
  ".wc3-slot",
  ".close-button"
];
var NAV_SELECTOR = NAV_SELECTORS.join(", ");
var PERPENDICULAR_WEIGHT = 2;
var TOLERANCE_PIXELS = 2;
var INSTALL_VERSION = 3;
var MIN_NAV_VIEWPORT_WIDTH = 1100;
var POINTER_FOCUSABLE_SELECTOR = [NAV_SELECTOR, "button", "[role='button']", "a[href]", "[tabindex]:not([tabindex='-1'])"].join(", ");
var COARSE_POINTER_MEDIA = "(hover: none), (pointer: coarse)";
var pointerModalityActive = false;
function isNavigationViewport() {
  return document.documentElement.clientWidth >= MIN_NAV_VIEWPORT_WIDTH;
}
function hasCoarsePointer() {
  return window.matchMedia(COARSE_POINTER_MEDIA).matches;
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
function installModalityTracking() {
  const setKeyboard = (event) => {
    if (!isNavigationViewport()) return;
    if (hasCoarsePointer()) {
      pointerModalityActive = true;
      document.body.removeAttribute("data-kb-modality");
      return;
    }
    const target = event.target;
    if (target instanceof HTMLElement) {
      const tag = target.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;
      if (target.isContentEditable) return;
    }
    pointerModalityActive = false;
    document.body.setAttribute("data-kb-modality", "");
  };
  const setPointer = () => {
    if (!isNavigationViewport()) return;
    pointerModalityActive = true;
    document.body.removeAttribute("data-kb-modality");
  };
  document.addEventListener("keydown", setKeyboard, true);
  document.addEventListener("mousedown", setPointer, true);
  document.addEventListener("mouseup", setPointer, true);
  document.addEventListener("pointerdown", setPointer, true);
  document.addEventListener("pointerup", setPointer, true);
  document.addEventListener("touchstart", setPointer, true);
  document.addEventListener("touchend", setPointer, true);
  document.addEventListener(
    "click",
    (event) => {
      if (event instanceof MouseEvent && event.detail === 0) return;
      setPointer();
    },
    true
  );
}
function installSpatialNavigation() {
  document.addEventListener(
    "keydown",
    (event) => {
      if (!isNavigationViewport()) return;
      if (event.ctrlKey || event.metaKey || event.altKey) return;
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
    true
  );
}
function installFocusAfterRender() {
  window.__focusAfterRender = (selector) => {
    if (!isNavigationViewport()) return;
    if (!document.body.hasAttribute("data-kb-modality")) return;
    if (hasCoarsePointer()) return;
    requestAnimationFrame(() => {
      const target = document.querySelector(selector);
      if (target) target.focus();
    });
  };
}
function installPointerFocusSuppression() {
  const shouldKeepFocus = (element) => {
    const tag = element.tagName;
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
    if (element.isContentEditable) return true;
    return false;
  };
  const inPointerModality = () => isNavigationViewport() && (pointerModalityActive || hasCoarsePointer() || !document.body.hasAttribute("data-kb-modality"));
  const blurActivePointerFocus = () => {
    if (!isNavigationViewport()) return false;
    if (!inPointerModality()) return false;
    const active = document.activeElement;
    if (!(active instanceof HTMLElement)) return false;
    const navEl = active.closest(POINTER_FOCUSABLE_SELECTOR);
    if (!(navEl instanceof HTMLElement)) return false;
    if (shouldKeepFocus(navEl)) return false;
    active.blur();
    document.body.removeAttribute("data-kb-modality");
    return true;
  };
  const guardAgainstPostRenderFocus = () => {
    blurActivePointerFocus();
    requestAnimationFrame(() => {
      blurActivePointerFocus();
      requestAnimationFrame(blurActivePointerFocus);
    });
  };
  document.addEventListener(
    "mousedown",
    (event) => {
      if (!isNavigationViewport()) return;
      if (!inPointerModality()) return;
      const target = event.target;
      if (!(target instanceof HTMLElement)) return;
      const navEl = target.closest(NAV_SELECTOR);
      if (navEl) event.preventDefault();
    },
    true
  );
  document.addEventListener("focusin", guardAgainstPostRenderFocus, true);
  document.addEventListener("pointerup", guardAgainstPostRenderFocus, true);
  document.addEventListener("touchend", guardAgainstPostRenderFocus, true);
  document.addEventListener("mouseup", guardAgainstPostRenderFocus, true);
  document.addEventListener(
    "click",
    (event) => {
      if (event instanceof MouseEvent && event.detail === 0) return;
      guardAgainstPostRenderFocus();
    },
    true
  );
}
if (!isNavigationViewport()) {
  window.__focusAfterRender = () => {
  };
} else if (window.__kbNavigationVersion !== INSTALL_VERSION) {
  window.__kbNavigationVersion = INSTALL_VERSION;
  window.__kbModalityInstalled = true;
  installModalityTracking();
  installSpatialNavigation();
  installFocusAfterRender();
  installPointerFocusSuppression();
}
