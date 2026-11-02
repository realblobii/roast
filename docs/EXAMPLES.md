# Examples

All **untested in the game**. Declare the blocks and models as in the [Quickstart](QUICKSTART.md).

## A roast chicken feast (4 servings, bowl, leaves a dish)

```json
{ "type": "roast:feast", "block": "mypack:roast_chicken_block", "max": 4, "container": "minecraft:bowl",
  "serving": "mypack:roast_chicken", "on_empty": { "replace": "mypack:roast_chicken_dish" }, "sound": "minecraft:block.wool.break" }
```

## A bread loaf you slice by hand (no container)

```json
{ "type": "roast:feast", "block": "mypack:loaf", "max": 6, "serving": "mypack:bread_slice", "on_empty": "break" }
```

## A pumpkin pie (4 bites, with a regeneration kick)

The block declares `bites` as an enum "0".."3". Use a plain template (such as `minecraft:stone`), not `minecraft:cake`.

```json
{ "type": "roast:cake", "block": "mypack:pumpkin_pie", "max": 3, "nutrition": 3, "saturation_modifier": 0.4,
  "effects": [ { "id": "minecraft:regeneration", "duration": 60 } ] }
```

## A pie you can eat from or slice with a knife (Pumpkin Pie Delight style)

```json
{ "type": "roast:cake", "block": "mypack:pumpkin_pie", "max": 3, "nutrition": 3, "saturation": 0.3,
  "slice_tool": "#c:tools/knife", "slice_item": "mypack:pumpkin_pie_slice" }
```

## A plate that shows any item

```json
{ "type": "roast:display", "block": "mypack:plate", "scale": 0.6, "offset": [0.5, 0.08, 0.5] }
```

## A display shelf for food only, standing up

```json
{ "type": "roast:display", "block": "mypack:shelf", "accepts": ["#c:foods"], "lay_flat": false, "scale": 0.5, "offset": [0.5, 0.5, 0.5] }
```

## Finger foods on a plate (Display Delight style): up to 4, sneak to place, empty hand takes one, sneak takes all

The block declares `count` as an enum "1".."4" and a blockstate model per count.

```json
{ "type": "roast:placeable", "item": "mypack:spring_roll", "block": "mypack:spring_rolls_plate",
  "stack_max": 4, "stack_property": "count", "sneak_to_place": true }
```

## A placeable cookie

```json
{ "type": "roast:placeable", "item": "mypack:cookie", "block": "mypack:cookie_block" }
```
Give `cookie_block` a loot table that drops `mypack:cookie`, and a blockstate model of one cookie.
