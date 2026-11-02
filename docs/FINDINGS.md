# Findings (Phase 0, first pass, 2 Nov 2026)

Pinned source: Horizon `c9d474dd9116e84c243f8992d75c11314df56f06` (the same pin as Cookpot). `hooks.rs` means
`crates/horizon-client/src/game/native_hooks.rs`, `server.rs` means `crates/horizon-client/src/game/native_server.rs`.
**Nothing here has been built or run.** This is a first pass from reading source; several answers are open.

## Answered (from source)

**1. Right-click, held item, give and take: yes.**
- `native_right_click_block(pos, face, canceled)` runs before the block's own use and can cancel the click, once per hand,
  server-side for every player (`hooks.rs:2434-2441`, API 21). `native_acting_player` says who.
- `native_held()` gives the main hand's item, count and whether it is plain (`server.rs:633`); `native_take_items(item, n)`
  (`server.rs:658`), `native_give_items(item, n)` (`server.rs:683`) and `native_give_stack` (`server.rs:708`) change the inventory.
- `native_use_item_on(stack, pos, face, claimed)` can claim an item's use on a block (`hooks.rs:2292`, API 13).

**2. Changing state at run time: yes.** `native_set_block_property(pos, key, value)` (`hooks.rs:3356`, API 18; the tick-context
version at `hooks.rs:3036`, API 33) returns false when the block lacks the property or value. `native_block_property(pos, key)`
reads one (`hooks.rs:3686`). `native_set_block` (`hooks.rs:3181`) replaces a block.

**4. Enum state properties: yes (per the docs).** `state_properties` accepts `{"type": "enum", "values": [...]}` with values of lower
case letters, digits and underscores, so "0".."7" is allowed (`docs/mods.md`, content-mod section). Its states multiply the
template's. Whether the blockstate file can key models on those values is expected (it is a vanilla blockstate) but **untested**.

**9. Discovery:** Cookpot's route works for Roast too: recipe files of an unknown `type` are kept as raw JSON and returned by
`native_recipes_of_type(kind)` (`hooks.rs:3861`, API 36). Roast files use `roast:*` types, so they cannot collide with `cookpot:*`.

## Partly answered

**5. Plates (an item shown in a block):** Cookpot's findings cite `native_block_entity_container` (a one-slot container on a block
whose template has a block entity) and `native_block_entity_submit` with `native_draw_item` (API 71) for drawing an item in the
world. Not re-read for Roast: confirm the draw call, its position arguments, and that a plate can use a `minecraft:barrel`-like
template without opening a menu (a one-slot content menu would open instead; may need `menu: None`).

**6. Placeable food:** `native_place_block(stack, pos, state)` (`hooks.rs:2419`, API 14) hears a placement; `native_use_item_on`
can claim a use and `native_set_block` can then place the block. Not confirmed: giving the item back when the block is broken
(a loot table with the item as its drop should do it).

## Open: likely blockers

**3. Feeding the player and applying effects.** No documented helper was found. `native_give_experience` exists for XP
(`server.rs:830`) but nothing for food level or effects. The game has `FoodData::eat_properties(nutrition, saturation)`
(`crates/horizon-world/src/food.rs:51`) and `potions.effects.add(...)` on the game (used by `beacon.rs:303`), so a hook compiled
in the game module might reach them through **private fields**; their names on the player's `Game` and their stability across
nightlies are unchecked. Until confirmed:
- **Feasts and placeable food can avoid this** by giving the player the normal food item (eaten the usual way).
- **Cakes that must feed directly cannot be done cleanly.** The fallback is the `minecraft:cake` template, whose eating
  behaviour has fixed nutrition. **Per the project rules, if no clean route exists this must go to the user before anything is built.**

**7, 8, 10.** Not started.

## Proposed first build (after the user's go-ahead)

A throwaway test mod with one feast block (`servings` 0 to 4, a model per state), and the smallest Roast: the right-click hook,
the held-item check, take-bowl and give-serving, set-property, replace at 0. Acceptance: right-click with a bowl four times gives
four servings and the block goes down a stage each time.
