# Roast

Roast lets [Horizon](https://github.com/horizon-rust/horizon) content mods add **feasts, cakes and pies, display plates and
placeable food** with JSON files only, no Rust. It adds no food or blocks of its own: it is the behaviour, and your mod brings the
blocks, models and recipes. (Its sister library for crafting machines is [Cookpot](https://github.com/realblobii/cookpot).)

| Kind | What it does |
|---|---|
| `roast:feast` | A block with servings. Right-click with a bowl (or any container you choose) to take one as an item. |
| `roast:cake` | A block eaten in bites, with nutrition, effects and a leftover. Optionally sliced with a knife. |
| `roast:display` | A block that holds and shows any item you put on it: a plate, a shelf, a stand. |
| `roast:placeable` | An item that can be set down as a block, can stack (finger foods on a plate) and be taken back. |

```json
{ "type": "roast:feast", "block": "mypack:roast_chicken_block", "max": 4, "container": "minecraft:bowl",
  "serving": "mypack:roast_chicken", "on_empty": "break" }
```

## Documentation

Start at the **[documentation home](docs/README.md)**:

- [Quickstart](docs/QUICKSTART.md): a working feast in four files
- [Authoring reference](docs/AUTHORING.md): every field of every kind
- [Placeable food guide](docs/PLACEABLES.md): stacking, take-back and drops, with a worked example
- [Examples](docs/EXAMPLES.md): feast, pie, plate, shelf, cookie
- [Shipping your mod](docs/SHIPPING.md) and [Troubleshooting](docs/TROUBLESHOOTING.md)
- [JSON schemas](schemas/) for editor validation

Real-world example: [Delightful](https://github.com/realblobii/delightful) uses Roast for its feasts, pies and 104 placeable foods, and doubles as the test mod.

## Status

**Untested in the game.** The logic has unit tests, but the game hooks have not yet been run in Horizon. Singleplayer is the target.

## Contributing

`mixins/roast/` is the native mod. Notes for people working on Roast itself are in [docs/dev/](docs/dev/).
