# Roast authoring reference (v0.1.0)

> **Written from the source and unit-tested only for its pure logic (against a stand-in JSON library). The game hooks were never
> compiled against Horizon or run.** See `FINDINGS.md` and `TROUBLESHOOTING.md`.

All four kinds are JSON files at `data/<ns>/recipe/<name>.json` with a `type`. The block (or item) itself is an ordinary content-mod
declaration; Roast adds the behaviour. A block named by a definition never gets vanilla's own use (so a cake-template block does not
eat twice), and only the main hand acts.

## `roast:feast`: servings you take

| Field | Default | Meaning |
|---|---|---|
| `block` | required | A block id, or a list of ids. |
| `property` | `servings` | The block's enum property counting servings **left** ("0".."max"). |
| `max` | 4 | The largest servings value. |
| `container` | none | The item the player must hold (`bowl`). Without it, any hand (even empty) takes a serving. |
| `consume_container` | true | Use the container up (a bowl) or keep it (a ladle). |
| `serving` | required | The item one serving gives. If the inventory is full nothing is taken. |
| `on_empty` | `keep` | When the last serving is taken: `keep` (property 0), `break` (remove the block), or `{ "replace": "ns:block" }`. |
| `clear_when_empty` | true | With `keep` and property 0, an empty hand removes the block. |
| `leftover` | none | Given when the empty block is cleared. |
| `sound` | none | A sound id played on a serving. |

## `roast:cake`: bites you eat

| Field | Default | Meaning |
|---|---|---|
| `block` | required | |
| `property` | `bites` | The enum property counting bites **eaten** ("0".."max"). |
| `max` | 6 | Eating the bite at `max` finishes it. |
| `nutrition` | 2 | Hunger restored per bite. |
| `saturation` or `saturation_modifier` | 0 | Saturation per bite, or nutrition x modifier x 2. |
| `always_edible` | false | Eat even when not hungry. |
| `effects` | none | List of `{ "id", "duration" (ticks), "amplifier", "chance" }` applied each bite (vanilla effect ids). |
| `leave` | none | A block that replaces the cake when finished (a plate); default: removed. |
| `sound` | none | |

Feeding and effects use the game's own food data and effect list, which are **private game API**: a Horizon nightly may move them.
They act on the clicking player in single player and for the acting player on a LAN host (untested).

## `roast:display`: a block that shows any item

The block holds **one item of any kind**, placed with a right-click (an empty hand takes it back). The block must be declared with a
template that has a block entity (`minecraft:barrel`, `chest` or `furnace`); Roast gives it one slot, no menu and no hopper access.

| Field | Default | Meaning |
|---|---|---|
| `block` | required | |
| `accepts` | anything | Item ids and `#tags` allowed; empty or absent allows any item. |
| `offset` | `[0.5, 0.1, 0.5]` | Where the item's centre is, in blocks from the block's lower north-west corner. |
| `scale` | 0.7 | Size, 0.05 to 4. |
| `rotation` | 0 | Degrees about the vertical axis. |
| `lay_flat` | true | Lay flat items on the surface; false stands them up. |
| `sound` | none | |

The item is drawn by the game as it draws items in the world. Only the item id is shown: components (a named item) are kept in the
block but not drawn.

## `roast:placeable`: an item set down as a block

| Field | Default | Meaning |
|---|---|---|
| `item` | required | The item. |
| `block` | required | The block placed in the air next to the clicked face. |
| `top_only` | true | Only on the top of a block; false allows any face. |
| `sound` | none | |

One item is used (none in creative). Breaking the block drops what its loot table says: give it a table that drops the item.

## Limits

- Definitions are read once when a world loads; edit JSON, restart the game. No Roast rebuild is needed.
- Feast servings leave the block's property only; models per value come from your blockstate file.
- A block must exist in a content mod; vanilla blocks cannot be given servings.
- Clients and servers need the same mods.
- Not yet: several items per display, shelf-style multi-slot displays, hunger restored by feasts directly (they give an item to eat),
  per-serving model parts drawn by Roast, comparator output.
