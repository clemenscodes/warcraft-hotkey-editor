# e2e tests — band layout

Specs are grouped by the responsive **band** they target. The six bands are the
disjoint width ranges defined once in `crates/hotkey-editor/tailwind.css` (see
`docs/COMPONENTS.md` → "Responsive bands"). Because nothing cascades across
bands, a spec that asserts layout must run at a viewport inside the band it
targets — that is what these directories guarantee.

```
tests/
  mobile/    < 768px      phones
  tablet/    768–1279px   portrait tablets
  laptop/    1280–1919px  laptops
  desktop/   1920–2559px  FHD desktops   ← the existing suite
  qhd/       2560–3839px  1440p
  uhd/       ≥ 3840px     4K
```

`playwright.config.ts` generates one project per `band × browser` (chromium,
firefox, webkit), scoping each to its `tests/<band>/` directory at that band's
viewport. A band directory with no `*.spec.ts` yet generates no project, so the
empty slots stay quiet until you add specs.

**To add a band-specific test:** drop `your-feature.spec.ts` into the matching
band directory. It automatically runs on all three browsers at that band's
viewport — no config change needed.

Behavioral tests that are genuinely band-agnostic (import/export, persistence,
templates, undo) live under `desktop/` and run once at the desktop baseline
rather than being multiplied across every band.
