// Draws the shown item on the display: laid flat on the surface unless the definition stands it up.
fn roast_submit(game: &crate::game::Game<'_>, block: &str, entity: &ModBlockEntity, draw: &mut BlockEntityDraw) {
    if entity.id != roast::ENTITY_ID {
        return;
    }
    let key = horizon_world::nbt::JavaString::from("shown");
    let Some(horizon_world::nbt::Tag::String(id)) = entity.nbt.get(&key) else {
        return;
    };
    let Some(defs) = roast::defs(|| roast::FILE_TYPES.iter().flat_map(|t| game.native_recipes_of_type(t).iter().map(|r| (r.id.clone(), (*r.json).clone()))).collect()) else {
        return;
    };
    let Some(d) = defs.display(block) else {
        return;
    };
    let stack = ItemStack::new(&id.to_string_lossy(), 1);
    draw.push();
    draw.translate(d.offset);
    if d.rotation != 0.0 {
        draw.rotate_about([0.0, 1.0, 0.0], d.rotation, [0.0, 0.0, 0.0]);
    }
    if d.lay_flat {
        draw.rotate_about([1.0, 0.0, 0.0], 90.0, [0.0, 0.0, 0.0]);
    }
    draw.scale([d.scale, d.scale, d.scale]);
    game.native_draw_item(draw, &stack, horizon_assets::item_model::DisplayContext::Fixed, None, 0);
    draw.pop();
}
