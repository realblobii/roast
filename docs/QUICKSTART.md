# Quickstart: a feast in four files

> Written from the source, **not run in the game**. A larger finished example is [Delightful](https://github.com/realblobii/delightful).

You need Roast installed (a native mod, built once by the launcher) and a content mod. Replace `mypack`.

## 1. The block

`data/mypack/horizon/block/stew_feast.json` declares a block with a `servings` count. A content block may declare enum properties;
the values here are the servings left, "0" to "4":

```json
{
  "template": "minecraft:stone",
  "item": true,
  "state_properties": { "servings": { "type": "enum", "values": ["0", "1", "2", "3", "4"] } },
  "default_state": { "servings": "4" }
}
```

## 2. The look: one model per servings value

`assets/mypack/blockstates/stew_feast.json`

```json
{ "variants": {
  "servings=4": { "model": "mypack:block/stew_feast_4" },
  "servings=3": { "model": "mypack:block/stew_feast_3" },
  "servings=2": { "model": "mypack:block/stew_feast_2" },
  "servings=1": { "model": "mypack:block/stew_feast_1" },
  "servings=0": { "model": "mypack:block/stew_feast_empty" } } }
```

## 3. What one serving is

`data/mypack/horizon/item/stew_serving.json`: an ordinary food item.

```json
{ "template": "minecraft:bread", "components": { "minecraft:food": { "nutrition": 6, "saturation": 7.2 } },
  "crafting_remainder": "minecraft:bowl" }
```

## 4. The Roast definition

`data/mypack/recipe/stew_feast.json` (it lives under `recipe/` because the game keeps recipe files of an unknown `type` as raw JSON;
that is how Roast finds it):

```json
{ "type": "roast:feast", "block": "mypack:stew_feast", "max": 4, "container": "minecraft:bowl",
  "serving": "mypack:stew_serving", "on_empty": { "replace": "mypack:empty_dish" } }
```

## Try it

Right-click the block with a bowl: the bowl is used up, you get a serving, and the block drops one stage. At the last serving it becomes
`mypack:empty_dish` (or stays, or is removed: see `on_empty`).

Next: [cakes, displays and placeable items](AUTHORING.md).
