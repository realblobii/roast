# Shipping a mod that uses Roast

1. Put your definitions in your content mod (`.hmod`) at `data/<namespace>/recipe/<name>.json`, with a `type` of `roast:feast`,
   `roast:cake`, `roast:display` or `roast:placeable`. Roast finds them as recipe files of an unknown type, the way Cookpot does.
2. Declare your blocks and items as in any content mod, with the right `state_properties` (`servings`, `bites`, a count).
3. **Tell players Roast must be installed** (and enabled under Options > Mods). Horizon's manifest has no documented dependency field,
   so say it in your description and README. Without Roast the blocks exist but do nothing. The log of a game with Roast shows `roast` lines.
4. Content is read once at game start: JSON edits need a game restart, not a Roast rebuild.
5. If something fails, compare with [Delightful](https://github.com/realblobii/delightful), which uses all four kinds.
6. Keep your block and item ids stable: players' worlds store them.

Validate files against the [schemas](../schemas/) in your editor by pointing `$schema` at the matching file.
