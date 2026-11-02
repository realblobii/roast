# Open questions (answer in FINDINGS.md, with a source line each)

## Blockers

1. Can a mod react to a right-click on its own block, read what the player holds, take one of it and give another item, and
   cancel the vanilla click? (Which hooks, and from which thread.)
2. Can a native mod change a block's state property at run time (servings 3 to 2), and replace the block when it reaches 0?
3. **Can a native mod feed the player (nutrition, saturation) and apply status effects?** (Needed for cakes; feasts can avoid it
   by giving a normal food item.) If only private game fields reach it, say which and how stable they look.
4. Can a content-mod block have a state property with more values than a boolean (an enum "0".."7"), and does a blockstate
   file pick a model per value?

## Plates and placeable food

5. Can a block entity hold one item (a container with one slot) and draw it in the block? (Cookpot's findings point at
   `native_block_entity_submit` and `native_draw_item`: confirm the draw call and the position it takes.)
6. Can a food item be placed as a block with `native_use_item_on` or `native_place_block`: which hook decides, and how does the
   result drop and return the item when broken?
7. Does breaking a feast or plate drop the right things (loot table with state conditions)?

## Practical

8. Which Nightly does the user have, and does it still contain every hook needed (hooks are versioned)?
9. How does Roast find definitions at run time and avoid colliding with Cookpot's files?
10. Delightful's 22 feast blocks: what do their blockstates need (properties, models per serving)?
