# Roast: brief

## Why

Horizon content mods copy a vanilla block as a template and can add enum or boolean state properties, but nothing gives a block
"servings", bites or a held item. Farmer's Delight's feasts, pies, and plates need exactly that. Roast provides it as data.

## The four kinds (draft: not confirmed against the source)

All are `type: roast:*` files under `data/<ns>/recipe/` (discovered the way Cookpot's files are), with the block declared as an
ordinary content-mod block.

| Kind | What it does | Needs (to verify) |
|---|---|---|
| `roast:feast` | `servings` state (an enum "0".."N"); right-click with the container item takes a serving: the item is taken, the serving item given, the state goes down; at 0 the block becomes another block or breaks | right-click hook, held-item helpers, set-property |
| `roast:cake` | `bites` state; right-click eats a bite: nutrition, saturation and effects from the definition; last bite leaves another block or nothing | the above plus a way to feed the player |
| `roast:plate` | a plate block that holds one item: right-click with an item puts it on, right-click empty takes it; the item is drawn on the plate | a block entity with an item slot, item drawing |
| `roast:placeable` | a food item that can be set down as a block (and eaten from it) | the use-item hook, place-block |

A feast block should be able to show the food on the plate, drop its container when emptied, and give a different serving item
per stage. Delightful's 22 feast blocks and pies are the acceptance test.

## Phases

0. Research (this folder's `OPEN_QUESTIONS.md`): feasibility, with evidence.
1. Smallest vertical slice: one feast block with servings that can be taken, in a throwaway test mod.
2. Data-driven definitions and validation; `roast:cake`.
3. Plates and placeable food (block entity and item drawing).
4. Docs for authors (quickstart, reference, examples, troubleshooting, JSON schemas) and a Delightful integration branch.

## Non-goals (for now)

Fluids, multiplayer polish, recipe viewers, and any content of its own.
