use std::ops::Deref;
use rand::{Rng, StdRng};
use game::{KnowledgeRenderer, Shadowcast, ActionEnv};
use ecs_core::EntityId;
use ecs_content::{EcsCtx, EcsAction, Entity, Ecs, EcsMut};
use engine_defs::{LevelId, EntityIdReserver};
use spatial_hash::SpatialHashTable;
use control::Frame;
use message::Language;

fn animation_policy(ecs: &mut EcsCtx,
                    action: &mut EcsAction,
                    _spatial_hash: &mut SpatialHashTable,
                    _entity_ids: &EntityIdReserver,
                    rng: &mut StdRng,
                    frame: Frame) -> bool {

    if frame.count() % 20 == 0 {
        for id in ecs.id_iter_acid_animation() {
            // don't always change every tile
            if rng.next_f64() > 0.5 {
                continue;
            }

            let animation = ecs.get_probabilistic_animation(id).expect("Entity missing probabilistic_animation");
            let tile = *animation.choose(rng);
            action.insert_tile(id, tile);
        }

        true
    } else {
        false
    }
}

fn render_frame<K: KnowledgeRenderer>(ecs: &mut EcsCtx,
                                      spatial_hash: &mut SpatialHashTable,
                                      entity_id: EntityId,
                                      level_id: LevelId,
                                      action_id: &mut u64,
                                      observer: &Shadowcast,
                                      renderer: &mut K,
                                      language: &Box<Language>) {
    let pc = ecs.entity(entity_id);

    let mut knowledge = pc.borrow_mut_drawable_knowledge()
        .expect("PC missing drawable_knowledge");

    let level_knowledge = knowledge.level_mut_or_insert_size(level_id,
                                                             spatial_hash.width(),
                                                             spatial_hash.height());
    let position = pc.copy_position().expect("PC missing position");
    let vision_distance = pc.copy_vision_distance().expect("PC missing vision_distance");
    let message_log = pc.borrow_message_log().expect("PC missing message_log");

    let action_env = ActionEnv::new(ecs, *action_id);
    let changed = observer.observe(position, spatial_hash, vision_distance, level_knowledge, action_env);

    if changed {
        renderer.update_and_publish_all_windows(*action_id, level_knowledge, position, message_log.deref(), &pc, language);
    }
}

pub fn animate_frame<K: KnowledgeRenderer>(ecs: &mut EcsCtx,
                                           action: &mut EcsAction,
                                           spatial_hash: &mut SpatialHashTable,
                                           entity_id: EntityId,
                                           level_id: LevelId,
                                           action_id: &mut u64,
                                           entity_ids: &EntityIdReserver,
                                           rng: &mut StdRng,
                                           observer: &Shadowcast,
                                           renderer: &mut K,
                                           language: &Box<Language>,
                                           frame: Frame) {

    let changed = animation_policy(ecs, action, spatial_hash, entity_ids, rng, frame);

    if !changed {
        return;
    }

    *action_id += 1;

    spatial_hash.update(ecs, action, *action_id);
    ecs.commit(action);

    render_frame(ecs, spatial_hash, entity_id, level_id, action_id, observer, renderer, language);
}
