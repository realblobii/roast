// A placeable item is set down as its block on the clicked face (the top, unless the definition allows any face).
fn roast_use_item(game: &mut crate::game::Game<'_>, stack: &ItemStack, pos: [i32; 3], face: [i32; 3], claimed: &mut bool) {
    let Some(defs) = roast::defs(|| roast::FILE_TYPES.iter().flat_map(|t| game.native_recipes_of_type(t).iter().map(|r| (r.id.clone(), (*r.json).clone()))).collect()) else {
        return;
    };
    let Some(p) = defs.placeable(&stack.item) else {
        return;
    };
    if game.hand != horizon_client::first_person::Hand::Main {
        return;
    }
    if p.sneak_to_place && !game.keys.contains(&KeyCode::ShiftLeft) {
        return;
    }
    let target = {
        let air_at = |at: [i32; 3]| game.native_block(at) == "minecraft:air";
        roast::placeable_target(p, pos, face, &air_at)
    };
    let Some(target) = target else {
        return;
    };
    let Some(id) = game.pack.assets.registry.default_state(&p.block) else {
        return;
    };
    let dim = game.dim.id().to_owned();
    if !game.native_set_block(&dim, target, id, flags::UPDATE_ALL) {
        return;
    }
    if p.stack_max > 1 {
        game.native_set_block_property(target, &p.stack_property, "1");
    }
    if !game.creative {
        game.native_take_items(&p.item, 1);
    }
    *claimed = true;
    if let Some(s) = &p.sound {
        game.native_play_local_sound(s, [f64::from(target[0]) + 0.5, f64::from(target[1]) + 0.5, f64::from(target[2]) + 0.5], 1.0, 1.0, 0);
    }
}
