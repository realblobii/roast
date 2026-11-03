// Roast's pure logic: feast, cake, display and placeable definitions read from content mods, and the decisions a click makes.
// No game types, so it is also a plain crate (`cargo test` at the repository root).
use std::sync::{Arc, Mutex};

pub const FEAST_TYPE: &str = "roast:feast";
pub const CAKE_TYPE: &str = "roast:cake";
pub const DISPLAY_TYPE: &str = "roast:display";
pub const PLACEABLE_TYPE: &str = "roast:placeable";
/// Every file type Roast reads.
pub const FILE_TYPES: [&str; 4] = [FEAST_TYPE, CAKE_TYPE, DISPLAY_TYPE, PLACEABLE_TYPE];
/// The block entity id of every display.
pub const ENTITY_ID: &str = "roast:display";

fn qualify(name: &str) -> String {
    if name.contains(':') { name.to_owned() } else { format!("minecraft:{name}") }
}

fn text<'a>(json: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    json.get(key).and_then(|v| v.as_str())
}

fn number(v: &serde_json::Value) -> Option<f32> {
    v.as_f64().filter(|n| n.is_finite()).map(|n| n as f32)
}

/// Whether `id` is a sound event id: `namespace:path`, lower case.
pub fn valid_sound_id(id: &str) -> bool {
    let Some((ns, path)) = id.split_once(':') else { return false };
    let ok = |c: char, slash: bool| c.is_ascii_lowercase() || c.is_ascii_digit() || "_.-".contains(c) || (slash && c == '/');
    !ns.is_empty() && !path.is_empty() && ns.chars().all(|c| ok(c, false)) && path.chars().all(|c| ok(c, true))
}

fn parse_sound(json: &serde_json::Value) -> Result<Option<String>, String> {
    match text(json, "sound") {
        None => Ok(None),
        Some(s) if valid_sound_id(s) => Ok(Some(s.to_owned())),
        Some(s) => Err(format!("`sound` `{s}` is not a sound id like `minecraft:block.wool.break`")),
    }
}

fn block_of(json: &serde_json::Value) -> Result<Vec<String>, String> {
    let blocks: Vec<String> = match json.get("block") {
        Some(serde_json::Value::String(b)) => vec![b.clone()],
        Some(serde_json::Value::Array(list)) => list.iter().filter_map(|b| b.as_str()).map(str::to_owned).collect(),
        _ => Vec::new(),
    };
    if blocks.is_empty() || blocks.iter().any(|b| !b.contains(':')) {
        return Err("`block` must be a block id like `mymod:roast_chicken` (or a list of them)".into());
    }
    Ok(blocks)
}

fn int(json: &serde_json::Value, key: &str, default: i64, min: i64, max: i64) -> Result<i32, String> {
    match json.get(key) {
        None => Ok(default as i32),
        Some(v) => v.as_i64().filter(|n| (min..=max).contains(n)).map(|n| n as i32).ok_or(format!("`{key}` must be a whole number from {min} to {max}")),
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Feast
// ---------------------------------------------------------------------------------------------------------------------

/// What happens to a feast block when its last serving is taken.
#[derive(Clone, Debug, PartialEq)]
pub enum Empty {
    /// The block stays, with its property at 0 (its model shows an empty dish).
    Keep,
    /// The block is removed.
    Break,
    /// The block becomes another block (an empty dish block).
    Replace(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Feast {
    pub id: String,
    pub blocks: Vec<String>,
    /// The block property that counts servings left (an enum "0".."max" the block declares).
    pub property: String,
    pub max: i32,
    /// The item a player must hold to take a serving; None: any hand, even an empty one.
    pub container: Option<String>,
    /// Whether the container is used up (a bowl is) or kept (a ladle is).
    pub consume_container: bool,
    /// The item a serving gives.
    pub serving: String,
    pub on_empty: Empty,
    /// With the property at 0 and `on_empty` Keep, a click with an empty hand removes the block.
    pub clear_when_empty: bool,
    /// Given when the block is cleared (a dish).
    pub leftover: Option<String>,
    pub sound: Option<String>,
}

pub fn parse_feast(id: &str, json: &serde_json::Value) -> Result<Feast, String> {
    let on_empty = match json.get("on_empty") {
        None => Empty::Keep,
        Some(serde_json::Value::String(s)) if s == "keep" => Empty::Keep,
        Some(serde_json::Value::String(s)) if s == "break" => Empty::Break,
        Some(v) => match text(v, "replace") {
            Some(b) if b.contains(':') => Empty::Replace(b.to_owned()),
            _ => return Err("`on_empty` is `keep`, `break` or `{ \"replace\": \"ns:block\" }`".into()),
        },
    };
    let serving = text(json, "serving").filter(|s| s.contains(':')).ok_or("`serving` must be an item id like `mymod:roast_chicken`")?;
    Ok(Feast {
        id: id.to_owned(),
        blocks: block_of(json)?,
        property: text(json, "property").unwrap_or("servings").to_owned(),
        max: int(json, "max", 4, 1, 64)?,
        container: text(json, "container").map(qualify),
        consume_container: json.get("consume_container").and_then(|v| v.as_bool()).unwrap_or(true),
        serving: serving.to_owned(),
        on_empty,
        clear_when_empty: json.get("clear_when_empty").and_then(|v| v.as_bool()).unwrap_or(true),
        leftover: text(json, "leftover").map(qualify),
        sound: parse_sound(json)?,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub enum FeastOutcome {
    /// Nothing for Roast to do (the held item is wrong): the click is still kept from vanilla.
    Nothing,
    /// Take a serving: `take_container` says whether to remove one container item, `new` the servings left.
    Take { new: i32, take_container: bool },
    /// Remove the empty block, giving `leftover` when set.
    Clear,
}

/// What a click on a feast with `servings` left does, for a player holding `held` (None: an empty hand).
pub fn feast_click(f: &Feast, servings: i32, held: Option<&str>) -> FeastOutcome {
    if servings <= 0 {
        return if f.clear_when_empty && f.on_empty == Empty::Keep && held.is_none() { FeastOutcome::Clear } else { FeastOutcome::Nothing };
    }
    match &f.container {
        Some(c) if held != Some(c.as_str()) => FeastOutcome::Nothing,
        container => FeastOutcome::Take { new: servings - 1, take_container: container.is_some() && f.consume_container },
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Cake
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    pub id: String,
    pub duration: i32,
    pub amplifier: i32,
    pub chance: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cake {
    pub id: String,
    pub blocks: Vec<String>,
    /// The property that counts bites already eaten: an enum "0".."max".
    pub property: String,
    pub max: i32,
    pub nutrition: i32,
    pub saturation: f32,
    pub always_edible: bool,
    pub effects: Vec<Effect>,
    /// Eating the bite at `max` removes the block; `leave` names a block that replaces it instead (a plate).
    pub leave: Option<String>,
    /// An item id or `#tag` (a knife) that cuts a bite off as an item instead of eating it.
    pub slice_tool: Option<String>,
    /// The item a cut bite gives (a slice of pie).
    pub slice_item: Option<String>,
    pub sound: Option<String>,
}

pub fn parse_cake(id: &str, json: &serde_json::Value) -> Result<Cake, String> {
    let mut effects = Vec::new();
    if let Some(list) = json.get("effects").and_then(|v| v.as_array()) {
        for e in list {
            let eid = text(e, "id").filter(|s| s.contains(':') || !s.is_empty()).ok_or("an effect needs an `id`")?;
            effects.push(Effect {
                id: qualify(eid),
                duration: int(e, "duration", 100, 1, 1_000_000)?,
                amplifier: int(e, "amplifier", 0, 0, 255)?,
                chance: e.get("chance").and_then(number).unwrap_or(1.0).clamp(0.0, 1.0),
            });
        }
    }
    let saturation = match (json.get("saturation").and_then(number), json.get("saturation_modifier").and_then(number)) {
        (Some(s), _) => s,
        (None, Some(m)) => int(json, "nutrition", 2, 0, 20)? as f32 * m * 2.0,
        (None, None) => 0.0,
    };
    Ok(Cake {
        id: id.to_owned(),
        blocks: block_of(json)?,
        property: text(json, "property").unwrap_or("bites").to_owned(),
        max: int(json, "max", 6, 1, 64)?,
        nutrition: int(json, "nutrition", 2, 0, 20)?,
        saturation: saturation.max(0.0),
        always_edible: json.get("always_edible").and_then(|v| v.as_bool()).unwrap_or(false),
        effects,
        leave: text(json, "leave").filter(|b| b.contains(':')).map(str::to_owned),
        slice_tool: text(json, "slice_tool").map(|t| if let Some(x) = t.strip_prefix('#') { format!("#{}", qualify(x)) } else { qualify(t) }),
        slice_item: text(json, "slice_item").map(qualify),
        sound: parse_sound(json)?,
    })
}

impl Cake {
    /// Whether `held` is the tool that cuts slices. `in_tag(item, tag)` tests an item tag (tag without `#`).
    pub fn is_slicer(&self, held: Option<&str>, in_tag: &dyn Fn(&str, &str) -> bool) -> bool {
        match (&self.slice_tool, &self.slice_item, held) {
            (Some(t), Some(_), Some(h)) => match t.strip_prefix('#') { Some(tag) => in_tag(h, tag), None => t == h },
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CakeOutcome {
    /// Cut a bite off as an item (`slice_item`); `new` is the bites gone after it, or None when it was the last.
    Slice { new: Option<i32> },
    /// Not hungry enough (and not `always_edible`).
    Full,
    /// Eat a bite; `new` is the bites eaten after it, or None when this bite finishes it and the block goes.
    Eat { new: Option<i32> },
}

/// `hungry` is whether the player's food level is below full.
pub fn cake_click(c: &Cake, bites: i32, hungry: bool, slicing: bool) -> CakeOutcome {
    if slicing {
        return CakeOutcome::Slice { new: if bites >= c.max { None } else { Some(bites + 1) } };
    }
    if !hungry && !c.always_edible {
        return CakeOutcome::Full;
    }
    CakeOutcome::Eat { new: if bites >= c.max { None } else { Some(bites + 1) } }
}

// ---------------------------------------------------------------------------------------------------------------------
// Display: a block that holds one item and shows it
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Display {
    pub id: String,
    pub blocks: Vec<String>,
    /// Item ids and `#tags` that may be put on it; empty means anything.
    pub accepts: Vec<String>,
    /// Where the item's centre is, in blocks from the block's lower north-west corner.
    pub offset: [f32; 3],
    pub scale: f32,
    /// Degrees about the vertical axis.
    pub rotation: f32,
    /// Flat items lie on the surface (rotated 90 degrees about x) instead of standing.
    pub lay_flat: bool,
    pub sound: Option<String>,
}

pub fn parse_display(id: &str, json: &serde_json::Value) -> Result<Display, String> {
    let accepts: Vec<String> = match json.get("accepts") {
        None => Vec::new(),
        Some(serde_json::Value::Array(list)) => list.iter().filter_map(|x| x.as_str()).map(|s| if let Some(t) = s.strip_prefix('#') { format!("#{}", qualify(t)) } else { qualify(s) }).collect(),
        Some(_) => return Err("`accepts` is a list of item ids and #tags".into()),
    };
    let offset = match json.get("offset").and_then(|v| v.as_array()).map(|a| a.as_slice()) {
        None => [0.5, 0.1, 0.5],
        Some([x, y, z]) => [number(x).ok_or("`offset` must be three numbers")?, number(y).ok_or("`offset` must be three numbers")?, number(z).ok_or("`offset` must be three numbers")?],
        Some(_) => return Err("`offset` must be [x, y, z]".into()),
    };
    let scale = json.get("scale").and_then(number).unwrap_or(0.7);
    if !(0.05..=4.0).contains(&scale) {
        return Err("`scale` must be 0.05 to 4".into());
    }
    Ok(Display {
        id: id.to_owned(),
        blocks: block_of(json)?,
        accepts,
        offset,
        scale,
        rotation: json.get("rotation").and_then(number).unwrap_or(0.0),
        lay_flat: json.get("lay_flat").and_then(|v| v.as_bool()).unwrap_or(true),
        sound: parse_sound(json)?,
    })
}

impl Display {
    /// Whether `item` may be put on this display. `in_tag(item, tag)` tests an item tag (tag without `#`).
    pub fn allows(&self, item: &str, in_tag: &dyn Fn(&str, &str) -> bool) -> bool {
        self.accepts.is_empty() || self.accepts.iter().any(|a| match a.strip_prefix('#') { Some(t) => in_tag(item, t), None => a == item })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayOutcome {
    Nothing,
    /// Put one of the held item on the empty display.
    Put,
    /// Take the shown item back into the player's hands or inventory.
    Take,
}

pub fn display_click(d: &Display, held: Option<&str>, shown: Option<&str>, in_tag: &dyn Fn(&str, &str) -> bool) -> DisplayOutcome {
    match (shown, held) {
        (None, Some(h)) if d.allows(h, in_tag) => DisplayOutcome::Put,
        (Some(_), None) => DisplayOutcome::Take,
        _ => DisplayOutcome::Nothing,
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Placeable: an item that is set down as a block
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Placeable {
    pub id: String,
    pub item: String,
    pub block: String,
    /// Only on the top of a block (default), or on any face.
    pub top_only: bool,
    /// Several of the item may share the block (finger foods on a plate): the most, counted by `stack_property`.
    pub stack_max: i32,
    /// The block's enum property counting the items ("1".."stack_max"); only used when `stack_max` is above 1.
    pub stack_property: String,
    /// The item is set down only while sneaking (so that using it normally still eats it).
    pub sneak_to_place: bool,
    /// Right-click with an empty hand takes one back (sneaking: all of them).
    pub take: bool,
    pub sound: Option<String>,
}

pub fn parse_placeable(id: &str, json: &serde_json::Value) -> Result<Placeable, String> {
    let item = text(json, "item").map(qualify).ok_or("`item` must be an item id")?;
    let block = text(json, "block").filter(|b| b.contains(':')).ok_or("`block` must be a block id")?;
    Ok(Placeable {
        id: id.to_owned(),
        item,
        block: block.to_owned(),
        top_only: json.get("top_only").and_then(|v| v.as_bool()).unwrap_or(true),
        stack_max: int(json, "stack_max", 1, 1, 64)?,
        stack_property: text(json, "stack_property").unwrap_or("count").to_owned(),
        sneak_to_place: json.get("sneak_to_place").and_then(|v| v.as_bool()).unwrap_or(false),
        take: json.get("take").and_then(|v| v.as_bool()).unwrap_or(true),
        sound: parse_sound(json)?,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlacedOutcome {
    Nothing,
    /// Add the held item to the block: `new` items on it afterwards.
    Add { new: i32 },
    /// Take one back: `new` items left (0 removes the block).
    TakeOne { new: i32 },
    /// Take all `count` back and remove the block.
    TakeAll { count: i32 },
}

/// What a click on a block a placeable put down does. `count` is the items on it (1 without a stack property).
pub fn placed_click(p: &Placeable, count: i32, held: Option<&str>, sneaking: bool) -> PlacedOutcome {
    match held {
        Some(h) if h == p.item && !sneaking && count < p.stack_max => PlacedOutcome::Add { new: count + 1 },
        None if p.take && sneaking => PlacedOutcome::TakeAll { count: count.max(1) },
        None if p.take => PlacedOutcome::TakeOne { new: count - 1 },
        _ => PlacedOutcome::Nothing,
    }
}

/// The position a placeable goes to, or None. `face` is the clicked face's normal; `air_at(pos)` says the spot is empty.
pub fn placeable_target(p: &Placeable, pos: [i32; 3], face: [i32; 3], air_at: &dyn Fn([i32; 3]) -> bool) -> Option<[i32; 3]> {
    if p.top_only && face != [0, 1, 0] {
        return None;
    }
    let target = [pos[0] + face[0], pos[1] + face[1], pos[2] + face[2]];
    air_at(target).then_some(target)
}

// ---------------------------------------------------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Defs {
    pub feasts: Vec<Feast>,
    pub cakes: Vec<Cake>,
    pub displays: Vec<Display>,
    pub placeables: Vec<Placeable>,
}

impl Defs {
    pub fn feast(&self, block: &str) -> Option<&Feast> {
        self.feasts.iter().find(|d| d.blocks.iter().any(|b| b == block))
    }
    pub fn cake(&self, block: &str) -> Option<&Cake> {
        self.cakes.iter().find(|d| d.blocks.iter().any(|b| b == block))
    }
    pub fn display(&self, block: &str) -> Option<&Display> {
        self.displays.iter().find(|d| d.blocks.iter().any(|b| b == block))
    }
    pub fn placeable(&self, item: &str) -> Option<&Placeable> {
        self.placeables.iter().find(|d| d.item == item)
    }
    /// The placeable whose block is `block`.
    pub fn placed(&self, block: &str) -> Option<&Placeable> {
        self.placeables.iter().find(|d| d.block == block)
    }
    /// Whether any definition names `block`.
    pub fn knows(&self, block: &str) -> bool {
        self.feast(block).is_some() || self.cake(block).is_some() || self.display(block).is_some() || self.placed(block).is_some_and(|p| p.take)
    }
}

/// Parses every file; a file that does not parse is skipped with a warning in the log.
pub fn load(files: Vec<(String, serde_json::Value)>) -> Defs {
    let mut defs = Defs::default();
    for (id, json) in files {
        let kind = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
        let result = match kind {
            FEAST_TYPE => parse_feast(&id, &json).map(|d| defs.feasts.push(d)),
            CAKE_TYPE => parse_cake(&id, &json).map(|d| defs.cakes.push(d)),
            DISPLAY_TYPE => parse_display(&id, &json).map(|d| defs.displays.push(d)),
            PLACEABLE_TYPE => parse_placeable(&id, &json).map(|d| defs.placeables.push(d)),
            _ => continue,
        };
        if let Err(e) = result {
            log::warn!("roast: {id}: {e}");
        }
    }
    defs
}

struct Registry {
    loaded: bool,
    defs: Option<Arc<Defs>>,
}

static REGISTRY: Mutex<Registry> = Mutex::new(Registry { loaded: false, defs: None });

/// The loaded definitions. `files` lists every Roast file (id and JSON) and is only called until some are found: it is empty
/// outside a world, and content mods never change while the game runs.
pub fn defs(files: impl FnOnce() -> Vec<(String, serde_json::Value)>) -> Option<Arc<Defs>> {
    let mut reg = REGISTRY.lock().ok()?;
    if !reg.loaded {
        let files = files();
        if files.is_empty() {
            return None;
        }
        reg.defs = Some(Arc::new(load(files)));
        reg.loaded = true;
    }
    reg.defs.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn feast() -> Feast {
        parse_feast("t:feast", &json!({ "type": "roast:feast", "block": "t:stew", "max": 4, "container": "bowl", "serving": "t:stew_bowl",
            "on_empty": { "replace": "t:dish" } })).unwrap()
    }

    fn no_tag(_: &str, _: &str) -> bool {
        false
    }

    #[test]
    fn feast_takes_a_serving_with_the_container() {
        let f = feast();
        assert_eq!(f.container.as_deref(), Some("minecraft:bowl"));
        assert_eq!(feast_click(&f, 4, Some("minecraft:bowl")), FeastOutcome::Take { new: 3, take_container: true });
        assert_eq!(feast_click(&f, 1, Some("minecraft:bowl")), FeastOutcome::Take { new: 0, take_container: true });
        assert_eq!(feast_click(&f, 4, Some("minecraft:stick")), FeastOutcome::Nothing);
        assert_eq!(feast_click(&f, 4, None), FeastOutcome::Nothing);
    }

    #[test]
    fn empty_feast_clears_with_an_empty_hand_when_kept() {
        let mut f = feast();
        assert_eq!(feast_click(&f, 0, None), FeastOutcome::Nothing); // replaced on empty, nothing left to clear
        f.on_empty = Empty::Keep;
        assert_eq!(feast_click(&f, 0, None), FeastOutcome::Clear);
        assert_eq!(feast_click(&f, 0, Some("minecraft:bowl")), FeastOutcome::Nothing);
    }

    #[test]
    fn a_feast_without_a_container_takes_servings_by_hand() {
        let f = parse_feast("t:f", &json!({ "block": ["t:a", "t:b"], "serving": "t:slice", "on_empty": "break" })).unwrap();
        assert_eq!(feast_click(&f, 2, None), FeastOutcome::Take { new: 1, take_container: false });
        assert!(f.blocks.len() == 2 && f.on_empty == Empty::Break);
    }

    #[test]
    fn bad_feasts_say_why() {
        assert!(parse_feast("x", &json!({ "block": "pot", "serving": "t:a" })).unwrap_err().contains("block"));
        assert!(parse_feast("x", &json!({ "block": "t:a" })).unwrap_err().contains("serving"));
        assert!(parse_feast("x", &json!({ "block": "t:a", "serving": "t:b", "on_empty": "explode" })).is_err());
        assert!(parse_feast("x", &json!({ "block": "t:a", "serving": "t:b", "sound": "Bad Sound" })).unwrap_err().contains("sound"));
    }

    #[test]
    fn cakes_are_eaten_in_bites_and_vanish_at_the_end() {
        let c = parse_cake("t:c", &json!({ "block": "t:pie", "max": 3, "nutrition": 3, "saturation_modifier": 0.5,
            "effects": [ { "id": "regeneration", "duration": 40 } ] })).unwrap();
        assert_eq!(c.saturation, 3.0);
        assert_eq!(c.effects[0].id, "minecraft:regeneration");
        assert_eq!(cake_click(&c, 0, true, false), CakeOutcome::Eat { new: Some(1) });
        assert_eq!(cake_click(&c, 3, true, false), CakeOutcome::Eat { new: None });
        assert_eq!(cake_click(&c, 0, false, false), CakeOutcome::Full);
        let mut a = c.clone();
        a.always_edible = true;
        assert_eq!(cake_click(&a, 0, false, false), CakeOutcome::Eat { new: Some(1) });
    }

    #[test]
    fn a_knife_cuts_slices_off_a_pie() {
        let c = parse_cake("t:c", &json!({ "block": "t:pie", "max": 3, "slice_tool": "#c:tools/knife", "slice_item": "t:slice" })).unwrap();
        let knife = |i: &str, t: &str| t == "c:tools/knife" && i == "t:knife";
        assert!(c.is_slicer(Some("t:knife"), &knife) && !c.is_slicer(Some("t:spoon"), &knife) && !c.is_slicer(None, &knife));
        assert_eq!(cake_click(&c, 1, false, true), CakeOutcome::Slice { new: Some(2) });
        assert_eq!(cake_click(&c, 3, false, true), CakeOutcome::Slice { new: None });
        let plain = parse_cake("t:c", &json!({ "block": "t:pie" })).unwrap();
        assert!(!plain.is_slicer(Some("t:knife"), &knife));
    }

    #[test]
    fn displays_hold_any_item_or_a_filtered_list() {
        let any = parse_display("t:d", &json!({ "block": "t:plate" })).unwrap();
        assert_eq!(display_click(&any, Some("minecraft:apple"), None, &no_tag), DisplayOutcome::Put);
        assert_eq!(display_click(&any, None, Some("minecraft:apple"), &no_tag), DisplayOutcome::Take);
        assert_eq!(display_click(&any, Some("minecraft:apple"), Some("minecraft:apple"), &no_tag), DisplayOutcome::Nothing);
        assert_eq!(display_click(&any, None, None, &no_tag), DisplayOutcome::Nothing);
        let food = parse_display("t:d", &json!({ "block": "t:plate", "accepts": ["#c:foods", "bread"], "scale": 0.5 })).unwrap();
        let tag = |i: &str, t: &str| t == "c:foods" && i == "minecraft:apple";
        assert_eq!(display_click(&food, Some("minecraft:apple"), None, &tag), DisplayOutcome::Put);
        assert_eq!(display_click(&food, Some("minecraft:bread"), None, &tag), DisplayOutcome::Put);
        assert_eq!(display_click(&food, Some("minecraft:stone"), None, &tag), DisplayOutcome::Nothing);
        assert!(parse_display("x", &json!({ "block": "t:p", "scale": 9 })).is_err());
    }

    #[test]
    fn placeables_go_on_top_of_blocks_into_air() {
        let p = parse_placeable("t:p", &json!({ "item": "t:cookie", "block": "t:cookie_block" })).unwrap();
        let air = |pos: [i32; 3]| pos == [1, 5, 1];
        assert_eq!(placeable_target(&p, [1, 4, 1], [0, 1, 0], &air), Some([1, 5, 1]));
        assert_eq!(placeable_target(&p, [1, 4, 1], [1, 0, 0], &air), None);
        let side = parse_placeable("t:p", &json!({ "item": "t:cookie", "block": "t:cookie_block", "top_only": false })).unwrap();
        assert_eq!(placeable_target(&side, [0, 5, 1], [1, 0, 0], &air), Some([1, 5, 1]));
        assert_eq!(placeable_target(&p, [1, 3, 1], [0, 1, 0], &air), None);
    }

    #[test]
    fn finger_foods_stack_and_come_back_one_or_all() {
        let p = parse_placeable("t:p", &json!({ "item": "t:cookie", "block": "t:cookies", "stack_max": 4, "sneak_to_place": true })).unwrap();
        assert!(p.sneak_to_place && p.take && p.stack_property == "count");
        assert_eq!(placed_click(&p, 1, Some("t:cookie"), false), PlacedOutcome::Add { new: 2 });
        assert_eq!(placed_click(&p, 4, Some("t:cookie"), false), PlacedOutcome::Nothing);
        assert_eq!(placed_click(&p, 3, Some("t:other"), false), PlacedOutcome::Nothing);
        assert_eq!(placed_click(&p, 3, None, false), PlacedOutcome::TakeOne { new: 2 });
        assert_eq!(placed_click(&p, 1, None, false), PlacedOutcome::TakeOne { new: 0 });
        assert_eq!(placed_click(&p, 3, None, true), PlacedOutcome::TakeAll { count: 3 });
        let mut keep = p.clone();
        keep.take = false;
        assert_eq!(placed_click(&keep, 3, None, false), PlacedOutcome::Nothing);
        assert!(parse_placeable("x", &json!({ "item": "t:a", "block": "t:b", "stack_max": 99 })).is_err());
    }

    #[test]
    fn registry_loads_files_of_all_four_kinds_and_skips_bad_ones() {
        let defs = load(vec![
            ("t:a".into(), json!({ "type": "roast:feast", "block": "t:stew", "serving": "t:s" })),
            ("t:b".into(), json!({ "type": "roast:cake", "block": "t:pie" })),
            ("t:c".into(), json!({ "type": "roast:display", "block": "t:plate" })),
            ("t:d".into(), json!({ "type": "roast:placeable", "item": "t:cookie", "block": "t:cookie_block" })),
            ("t:bad".into(), json!({ "type": "roast:feast" })),
            ("t:other".into(), json!({ "type": "cookpot:machine" })),
        ]);
        assert_eq!((defs.feasts.len(), defs.cakes.len(), defs.displays.len(), defs.placeables.len()), (1, 1, 1, 1));
        assert!(defs.knows("t:plate") && defs.knows("t:pie") && defs.knows("t:cookie_block"));
        assert!(defs.placed("t:cookie_block").is_some() && defs.placed("t:nothing").is_none());
        assert!(defs.placeable("minecraft:nothing").is_none() && defs.placeable("t:cookie").is_some());
    }
}
