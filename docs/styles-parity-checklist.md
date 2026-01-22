# Styles Parity Checklist (Legacy PWA)

Use this checklist when porting legacy PWA UI into the Leptos app.

## Theme + Tokens
- [ ] `data-theme` toggles update tokens (`os_light` / `os_dark`).
- [ ] `prefers-color-scheme` applies only when no explicit theme override is set.
- [ ] Token overrides match legacy palette values for light/dark.

## DaisyUI Integration
- [ ] `@plugin "daisyui"` enabled with `os_light` / `os_dark` themes.
- [ ] DaisyUI theme blocks are present under `@plugin "daisyui/theme"`.
- [ ] DaisyUI color slots use `--fallback-*` mapped to app tokens.

## Tailwind Sources
- [ ] `@source` includes app HTML and Rust component paths.
- [ ] `@source` includes shared crate component paths.

## Utility Parity
- [ ] `apps-base.css` matches legacy resets and globals.
- [ ] `apps-ui.css` utility layers match legacy ordering and definitions.
- [ ] UI component helpers in `crates/ui-components/assets` align with legacy semantics.

## Visual Smoke Checks
- [ ] Light/dark toggle updates backgrounds, text, accents, and surfaces.
- [ ] Buttons, inputs, list rows, and sheets render with expected padding/radii.
- [ ] Default layout spacing matches legacy screens.
