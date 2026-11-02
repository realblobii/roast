// Mirrors the held item into the saved data (`shown`), which `update_tag` sends and the drawing reads.
fn roast_tick(pos: [i32; 3], entity: &mut ModBlockEntity) {
    let _ = pos;
    if entity.id != roast::ENTITY_ID {
        return;
    }
    let key = horizon_world::nbt::JavaString::from("shown");
    let item = entity.items.first().and_then(|s| s.as_ref()).map(|s| s.item.to_string());
    match item {
        Some(id) => {
            let tag = horizon_world::nbt::Tag::String(horizon_world::nbt::JavaString::from(id.as_str()));
            if entity.nbt.get(&key) != Some(&tag) {
                entity.nbt.insert(key, tag);
            }
        }
        None => {
            if entity.nbt.get(&key).is_some() {
                entity.nbt.remove(&key);
            }
        }
    }
}
