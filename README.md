# Roast

A **library native mod** for the [Horizon](https://github.com/horizon-rust/horizon) Minecraft client (an unofficial Rust
rewrite of Java Edition 26.3) that lets content mods add **feasts, placeable plates, placeable food and cake-like blocks**
with data files only, no Rust. Roast is to food blocks what [Cookpot](https://github.com/realblobii/cookpot) is to crafting
machines: it ships the behaviour and the documentation, and adds no food of its own.

[Delightful](https://github.com/realblobii/delightful) is the first user. It had to drop 22 feast blocks and turn its pies into
plain cakes because Horizon's block templates have no "servings" behaviour. Roast is what lets them come back.

## What it should cover

- **Feasts:** a block with servings. Right-click with the container item (a bowl) to take a serving, which becomes a normal
  food item in your inventory; the block shows fewer servings each time and leaves a dish or vanishes when empty.
- **Cake-like blocks:** a block eaten in bites (pies, cakes, cheesecakes), with its own nutrition, effects and leftover.
- **Displays:** a block that holds **any item** you place on it and shows it (a plate, a shelf, Display-Delight-style), not only cake.
- **Placeable food:** food items that can also be set down as a block and eaten from it.

All four are definitions in JSON inside a content mod; Roast reads them when a world loads, the way Cookpot does.

## Status

Version 0.3.0 is **written, not run in Horizon**: the pure logic has unit tests (against a stand-in JSON library); the game hooks were never
compiled against Horizon. The demo mod `testmod/rtest` shows a feast, a pie, a display and a placeable cookie.

## Files

- `docs/README.md`: the documentation home (quickstart, reference, examples, troubleshooting, schemas).
- `mixins/roast/`: the native mod. `testmod/`: the demo content mod. `schemas/`: JSON schemas.
- `CLAUDE.md`: rules for Claude working in this repo. `docs/BRIEF.md`, `OPEN_QUESTIONS.md`, `FINDINGS.md`: the research.