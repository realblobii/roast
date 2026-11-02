# Roast: guidance for Claude

You are building **Roast**, a **library** native Horizon mod: content mods define feasts, placeable plates, placeable food and
cake-like blocks with **data files and no Rust**. Roast adds no food or blocks of its own; examples are demos only.
`docs/AUTHORING.md` (to write) is a first-class deliverable, as in Cookpot.

Decisions already made (do not re-ask):
- Library mod, no content of its own. Delightful (`https://github.com/realblobii/delightful`) is the first user and the test case.
- Definitions ship inside content mods (`.hmod`), found the way Cookpot finds its files (recipe files of an unknown `type`).
- Singleplayer first; do not rule out multiplayer.
- If a required hook is missing, **stop and tell the user with evidence**. Do not silently work around it or invent a workaround.
- Follow Cookpot's conventions (`https://github.com/realblobii/cookpot`): pure logic in a `[[module]]` `runtime.rs` with unit
  tests, thin game glue in helpers, JSON schemas, a docs home page (`docs/README.md`), and honest "untested" labels.

## Current state

Roast 0.1.0 exists (written 2 Nov 2026, **never built or run in Horizon**). Read `docs/dev/FINDINGS.md` first. The four kinds are feast, cake, display (any item) and placeable.

## Rule 0: research before code

Do not write code until `docs/dev/OPEN_QUESTIONS.md` is answered in `docs/dev/FINDINGS.md` with the file and line each answer came from,
at the **pinned Horizon commit** (see Cookpot's `compatibility.json`; check it against the user's nightly each session).
Horizon is unofficial and moves fast: never rely on memory for a hook name, signature or file format. When docs and source
disagree, the source at the pinned commit wins. Never claim something works unless it was built and run in the game.

## Environment

- Get the Horizon source with `git clone https://github.com/horizon-rust/horizon` and `git fetch --depth 1 origin <commit>`;
  the build tools run in the user's launcher, not here (this sandbox cannot reach crates.io, so `cargo` cannot fetch crates).
- Native mod rules (horizon-mixins 0.6.0): `HEAD` hooks only for functions with closures; a helper file is exactly one private
  function; pure logic goes in the `[[module]]` runtime; bump the version in `horizon-mixin.toml` and `Cargo.toml` on every
  repackage. See Cookpot's `docs/LESSONS.md`.
- Compile-check pure logic with a stand-in `serde_json` if the real one is unavailable (Cookpot's session did this), and say so.

## Working with the user

- The user tests in the game and reports logs and screenshots. Keep iterations small and report built / not built, tested / untested.
- Do not publish or contact external services without asking. Do not push to `main` without being asked.
- Keep the docs current as features land.
