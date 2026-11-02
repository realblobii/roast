// A right-click on a Roast block: a feast's servings, a cake's bites, or a display's item. The click never reaches vanilla's own
// use of these blocks (a cake template would eat twice), and only the main hand acts.
fn roast_click(game: &mut crate::game::Game<'_>, pos: [i32; 3], face: [i32; 3], canceled: &mut bool) {
    let _ = face;
    let block = game.native_block(pos);
    let Some(defs) = roast::defs(|| roast::FILE_TYPES.iter().flat_map(|t| game.native_recipes_of_type(t).iter().map(|r| (r.id.clone(), (*r.json).clone()))).collect()) else {
        return;
    };
    if !defs.knows(&block) {
        return;
    }
    *canceled = true;
    if game.hand != horizon_client::first_person::Hand::Main {
        return;
    }
    let held: Option<String> = game.native_held().filter(|h| h.2).map(|h| h.0);
    // None: leave the block; Some(None): remove it; Some(Some(name)): replace it.
    let mut replace: Option<Option<String>> = None;
    let mut sound: Option<String> = None;

    if let Some(f) = defs.feast(&block) {
        let servings: i32 = game.native_block_property(pos, &f.property).and_then(|v| v.parse().ok()).unwrap_or(0);
        match roast::feast_click(f, servings, held.as_deref()) {
            roast::FeastOutcome::Nothing => {}
            roast::FeastOutcome::Take { new, take_container } => {
                let container = f.container.clone().unwrap_or_default();
                let took = !take_container || game.native_take_items(&container, 1) == 1;
                if took && game.native_give_items(&f.serving, 1) == 0 {
                    // No room for the serving: the container goes back.
                    if take_container {
                        game.native_give_items(&container, 1);
                    }
                } else if took {
                    if new == 0 {
                        replace = match &f.on_empty {
                            roast::Empty::Keep => {
                                game.native_set_block_property(pos, &f.property, "0");
                                None
                            }
                            roast::Empty::Break => Some(None),
                            roast::Empty::Replace(b) => Some(Some(b.clone())),
                        };
                    } else {
                        game.native_set_block_property(pos, &f.property, &new.to_string());
                    }
                    sound = f.sound.clone();
                }
            }
            roast::FeastOutcome::Clear => {
                replace = Some(None);
                if let Some(l) = &f.leftover {
                    game.native_give_items(l, 1);
                }
                sound = f.sound.clone();
            }
        }
    } else if let Some(c) = defs.cake(&block) {
        let bites: i32 = game.native_block_property(pos, &c.property).and_then(|v| v.parse().ok()).unwrap_or(0);
        let hungry = game.player.as_ref().is_some_and(|p| p.health.food.food < 20);
        if let roast::CakeOutcome::Eat { new } = roast::cake_click(c, bites, hungry) {
            // Feeding uses the game's own food data and effect list (private game API: a nightly may move them).
            if let Some(p) = game.player.as_mut() {
                let food = &mut p.health.food;
                food.food = (food.food + c.nutrition).clamp(0, 20);
                food.saturation = (food.saturation + c.saturation).clamp(0.0, food.food as f32);
            }
            for e in &c.effects {
                let roll = horizon_core::random::RandomSource::next_float(&mut game.world.random);
                if e.chance >= 1.0 || roll < e.chance {
                    if let Some(def) = horizon_client::brewing::effect(&e.id) {
                        game.add_player_effect(horizon_client::effects::EffectInstance::new(def.id, e.duration, e.amplifier));
                    }
                }
            }
            match new {
                Some(n) => {
                    game.native_set_block_property(pos, &c.property, &n.to_string());
                }
                None => replace = Some(c.leave.clone()),
            }
            sound = c.sound.clone();
        }
    } else if let Some(p) = defs.placed(&block) {
        let count: i32 = if p.stack_max > 1 { game.native_block_property(pos, &p.stack_property).and_then(|v| v.parse().ok()).unwrap_or(1) } else { 1 };
        let sneaking = game.keys.contains(&KeyCode::ShiftLeft);
        match roast::placed_click(p, count, held.as_deref(), sneaking) {
            roast::PlacedOutcome::Nothing => {}
            roast::PlacedOutcome::Add { new } => {
                if game.native_take_items(&p.item, 1) == 1 {
                    game.native_set_block_property(pos, &p.stack_property, &new.to_string());
                    sound = p.sound.clone();
                }
            }
            roast::PlacedOutcome::TakeOne { new } => {
                if game.native_give_items(&p.item, 1) == 1 {
                    if new <= 0 {
                        replace = Some(None);
                    } else {
                        game.native_set_block_property(pos, &p.stack_property, &new.to_string());
                    }
                    sound = p.sound.clone();
                }
            }
            roast::PlacedOutcome::TakeAll { count } => {
                let given = game.native_give_items(&p.item, count as u32) as i32;
                if given >= count {
                    replace = Some(None);
                } else if given > 0 {
                    game.native_set_block_property(pos, &p.stack_property, &(count - given).to_string());
                }
                if given > 0 {
                    sound = p.sound.clone();
                }
            }
        }
    } else if let Some(d) = defs.display(&block) {
        let shown: Option<String> = game
            .native_with_mod_block_entity(pos, |e| e.items.first().and_then(|s| s.as_ref()).map(|s| s.item.to_string()))
            .flatten();
        let outcome = {
            let in_tag = |item: &str, tag: &str| game.items().item_tags.get(tag).is_some_and(|t| t.contains(item));
            roast::display_click(d, held.as_deref(), shown.as_deref(), &in_tag)
        };
        match outcome {
            roast::DisplayOutcome::Nothing => {}
            roast::DisplayOutcome::Put => {
                let item = held.clone().unwrap_or_default();
                if game.native_take_items(&item, 1) == 1 {
                    let stack = ItemStack::new(&item, 1);
                    game.native_with_mod_block_entity(pos, |e| {
                        if e.items.is_empty() {
                            e.items.resize(1, None);
                        }
                        e.items[0] = Some(stack);
                    });
                    sound = d.sound.clone();
                }
            }
            roast::DisplayOutcome::Take => {
                let taken: Option<ItemStack> = game.native_with_mod_block_entity(pos, |e| e.items.get_mut(0).and_then(|s| s.take())).flatten();
                if let Some(stack) = taken {
                    let uuid = game.native_acting_id();
                    if let Some(rest) = game.native_give_stack(uuid, stack) {
                        // No room: it stays on the display.
                        game.native_with_mod_block_entity(pos, |e| e.items[0] = Some(rest));
                    } else {
                        sound = d.sound.clone();
                    }
                }
            }
        }
    }

    if let Some(target) = replace {
        let name = target.unwrap_or_else(|| "minecraft:air".to_owned());
        if let Some(id) = game.pack.assets.registry.default_state(&name) {
            let dim = game.dim.id().to_owned();
            game.native_set_block(&dim, pos, id, flags::UPDATE_ALL);
        }
    }
    if let Some(s) = sound {
        game.native_play_local_sound(&s, [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.5, f64::from(pos[2]) + 0.5], 1.0, 1.0, 0);
    }
}
