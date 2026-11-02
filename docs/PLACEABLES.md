# Guide: Display-Delight-style placeable food

This page builds a complete **stacking, placeable cookie** (the pattern Delightful uses for 104 foods) from scratch. Field meanings are in
the [reference](AUTHORING.md). **Untested in the game.**

You write five things: the block, its blockstate and models, its loot table, the Roast definition, and (optionally) a plate.

## 1. The block (`data/mypack/horizon/block/placed_cookie.json`)

Use a plain template, no block entity needed. `item: false` stops it becoming an item (the real item already exists).

```json
{
  "template": "minecraft:white_carpet",
  "item": false,
  "state_properties": {
    "facing": { "type": "enum", "values": ["north", "east", "south", "west"] },
    "stacks": { "type": "enum", "values": ["1", "2", "3", "4", "5", "6"] }
  },
  "default_state": { "facing": "north", "stacks": "1" }
}
```

The `stacks` property name must match `stack_property` below. Its values are the counts "1".."`stack_max`".

## 2. The look (`assets/mypack/blockstates/placed_cookie.json`)

One model per `stacks` value (add `facing` rotation with `y` if the model has a front):

```json
{ "variants": {
  "stacks=1": { "model": "mypack:block/placed_cookie_1" },
  "stacks=2": { "model": "mypack:block/placed_cookie_2" }
} }
```

List every value up to `stack_max`. A model can use any item texture or your own 3D model.

## 3. The drops (`data/mypack/loot_table/blocks/placed_cookie.json`)

Roast does not drop items itself; breaking the block uses this table. One entry per count, chosen with `block_state_property`
and sized with `set_count` (see Delightful's `placed_plated_cookie.json` for a finished copy). A single-item block just needs one
`minecraft:item` entry. The loot-table syntax for 26.3 is one of the unverified parts of the project: check it in the game log.

## 4. The Roast definition (`data/mypack/recipe/placed_cookie.json`)

```json
{
  "type": "roast:placeable",
  "item": "minecraft:cookie",
  "block": "mypack:placed_cookie",
  "top_only": true,
  "sneak_to_place": true,
  "stack_max": 6,
  "stack_property": "stacks",
  "take": true,
  "sound": "minecraft:block.wool.place"
}
```

- **`sneak_to_place: true`** keeps the food edible: normal right-click eats, Shift + right-click sets it down. Use `false` for
  items that cannot be used any other way.
- **Right-click the placed block with the same item** to add another (up to `stack_max`).
- **Empty hand** takes one back; **Shift + empty hand** takes them all.

## 5. Also let a plate show it

A [`roast:display`](AUTHORING.md#roastdisplay-a-block-that-shows-any-item) block shows any item on top of itself, so a plate or
shelf needs no per-food file. Declare the block on `minecraft:barrel`:

```json
{ "type": "roast:display", "block": "mypack:plate", "scale": 0.6, "offset": [0.5, 0.08, 0.5] }
```

## Generating many at once

Delightful's `tools/carry_dd.py` shows how to write these four files per food from a table; copy that approach if you have dozens.

## Not covered (versus the Java Display Delight)

No support table under non-sturdy tops, no axe-hides-the-plate, no facing toward the player (set `facing` yourself in the blockstate),
no wandering-trader sales, and no per-count models drawn by Roast: models always come from your blockstate.
