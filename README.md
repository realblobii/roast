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
- **Placeable plates:** a plate block that holds one item and shows it (a block entity that draws the item).
- **Placeable food:** food items that can also be set down as a block and eaten from it.

All four are definitions in JSON inside a content mod; Roast reads them when a world loads, the way Cookpot does.

## Status

Not started: this repository holds instructions and a first research pass (`docs/FINDINGS.md`). **Nothing has been built or run
in the game.**

## Files

- `CLAUDE.md`: rules for Claude working in this repo. Read first.
- `docs/BRIEF.md`: scope, the four kinds, a sketch of the data formats, phases.
- `docs/OPEN_QUESTIONS.md`: what the research must still answer.
- `docs/FINDINGS.md`: answers so far, with source lines.
