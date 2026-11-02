# Troubleshooting

Start with `logs/latest.log` and search for `roast`. A failed Roast build is in `~/.local/share/horizon-launcher/native/build.log`.
**Nothing here has been run in the game**, so some entries are predictions.

## Log messages

| Message (shortened) | Fix |
|---|---|
| `block must be a block id like ...` | `block` is missing or has no namespace. |
| `serving must be an item id` | A feast needs a `serving`. |
| `on_empty is keep, break or { "replace": ... }` | Fix the value. |
| `sound X is not a sound id` | Sound ids are `namespace:path`, lower case. |
| `X must be a whole number from A to B` | A number is out of range (`max` 1-64, `nutrition` 0-20). |
| `scale must be 0.05 to 4` | |

## Symptoms

**Clicking does nothing, and vanilla eats the block / opens something.** The block id in the definition must match the declared block
exactly, including its namespace.

**A feast never changes.** The block must declare the property (`servings` by default) as an enum with values "0".."max", and the
definition's `max` must equal the largest value. If the property is missing, Roast cannot change it.

**The serving is not given.** The inventory is full (the container is put back). Or the held item is not the `container`.

**A cake gives nothing or eats twice.** Use a plain template, not `minecraft:cake`. If it eats but gives no hunger, the game's food
fields may have moved in a nightly: check the build log and tell me.

**A display shows nothing.** The block's template needs a block entity (barrel, chest, furnace). Only the item id is drawn. Check the item
has a model.

**A placeable does nothing.** The clicked face must be the top (`top_only`), the space next to it must be air, and the block must exist.

**Edits to JSON have no effect.** Content is read once when a world loads: restart the game.
