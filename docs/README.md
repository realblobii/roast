# Roast documentation

Roast lets a content mod add **feasts, cakes and pies, displays that show any item, and placeable items** with JSON files only.

> **Status:** written from the source, **not yet run in the game**. Each page says what is untested.

| Page | Read it when |
|---|---|
| [Quickstart](QUICKSTART.md) | You want a working feast in four small files. Start here. |
| [Authoring reference](AUTHORING.md) | You need every field of the four kinds. |
| [Placeable food guide](PLACEABLES.md) | You want Display-Delight-style food set down as a 3D block, stacking, with a worked example. |
| [Shipping](SHIPPING.md) | You are releasing a mod that needs Roast. |
| [Examples](EXAMPLES.md) | You want to copy a feast, a pie, a plate or a display shelf, or a placeable cookie. |
| [Troubleshooting](TROUBLESHOOTING.md) | Something does not work, or the log shows a `roast` warning. |
| [JSON schemas](../schemas/) | You want your editor to check your files. |
| [Findings](FINDINGS.md), [Brief](BRIEF.md), [Open questions](OPEN_QUESTIONS.md) | You work on Roast itself. |

Working demo: [`testmod/rtest`](../testmod/rtest).

## Which file do I write?

| I want... | Write `type` |
|---|---|
| A block with servings you take with a bowl (a feast) | `roast:feast` |
| A block eaten in bites (a cake or pie) | `roast:cake` |
| A block that holds and shows **any item** you place on it (a plate, a shelf, a display) | `roast:display` |
| An item that is set down as a block | `roast:placeable` |
