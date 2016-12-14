use ecs::*;
use game::*;
use math::Coord;
use grid::{Grid, DynamicGrid};
use util::AnySet;

pub struct SpatialHashCell {
    // sum of opacities of everything in this cell
    opacity: f64,

    // count of solid entities in this cell
    solid: usize,

    // count of pc entities in this cell
    pc: usize,

    // count of moon entities in this cell
    moon: usize,

    // set of entities that are doors
    doors: AnySet<EntityId>,

    // set of entities currently in this cell
    entities: EntitySet,

    // action on which this cell was last updated
    last_updated: u64,
}

pub struct SpatialHashTable {
    grid: DynamicGrid<SpatialHashCell>,
    empty: SpatialHashCell,
}

impl SpatialHashCell {
    fn new() -> Self {
        SpatialHashCell {
            opacity: 0.0,
            solid: 0,
            pc: 0,
            moon: 0,
            doors: AnySet::new(),
            entities: EntitySet::new(),
            last_updated: 0,
        }
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }

    pub fn entity_ids(&self) -> &EntitySet {
        &self.entities
    }

    pub fn entity_id_iter(&self) -> EntitySetIter {
        self.entities.iter()
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    pub fn solid(&self) -> bool {
        self.solid != 0
    }

    pub fn moon(&self) -> bool {
        self.moon != 0
    }

    pub fn pc(&self) -> bool {
        self.pc != 0
    }

    pub fn any_door(&self) -> Option<EntityId> {
        self.doors.any()
    }
}

impl Default for SpatialHashCell {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialHashTable {
    pub fn new() -> Self {
        SpatialHashTable {
            grid: DynamicGrid::new(),
            empty: SpatialHashCell::new(),
        }
    }

    pub fn get(&self, coord: Coord) -> &SpatialHashCell {
        self.grid.get(coord).unwrap_or(&self.empty)
    }

    pub fn limits_min(&self) -> Coord {
        self.grid.limits_min()
    }

    pub fn limits_max(&self) -> Coord {
        self.grid.limits_max()
    }

    fn get_mut_with_default(&mut self, coord: Coord) -> &mut SpatialHashCell {
        self.grid.get_mut_with_default(coord)
    }

    fn change_entity_position(&mut self, entity: EntityRef, current_position: Coord, new_position: Coord, action_id: u64) {
        self.remove_entity_position(entity, current_position, action_id);
        self.add_entity_position(entity, new_position, action_id);
    }

    fn remove_entity_position(&mut self, entity: EntityRef, position: Coord, action_id: u64) {
        let mut cell = self.get_mut_with_default(position);
        if entity.contains_solid() {
            cell.solid -= 1;
        }
        if entity.contains_pc() {
            cell.pc -= 1;
        }
        if entity.contains_moon() {
            cell.moon -= 1;
        }
        if let Some(opacity) = entity.opacity() {
            cell.opacity -= opacity;
        }
        if entity.contains_door_state() {
            cell.doors.remove(entity.id());
        }
        cell.entities.remove(entity.id());
        cell.last_updated = action_id;
    }

    fn add_entity_position(&mut self, entity: EntityRef, position: Coord, action_id: u64) {
        let mut cell = self.get_mut_with_default(position);
        if entity.contains_solid() {
            cell.solid += 1;
        }
        if entity.contains_pc() {
            cell.pc += 1;
        }
        if entity.contains_moon() {
            cell.moon += 1;
        }
        if let Some(opacity) = entity.opacity() {
            cell.opacity += opacity;
        }
        if entity.contains_door_state() {
            cell.doors.insert(entity.id());
        }
        cell.entities.insert(entity.id());
        cell.last_updated = action_id;
    }

    fn update_solid(&mut self, action_env: ActionEnv, action: &EcsAction) {
        for entity_id in action.solid_positive_iter(action_env.ecs) {
            let entity = action_env.ecs.post_action_entity(entity_id, action);
            if let Some(position) = entity.position() {
                if !entity.current_contains_solid() {
                    // entity is becoming solid
                    let cell = self.get_mut_with_default(position);
                    cell.solid += 1;
                    cell.last_updated = action_env.id;
                }
            }
        }

        for entity_id in action.solid_negative_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            if let Some(position) = entity.position() {
                if entity.contains_solid() {
                    // removing solid from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.solid -= 1;
                    cell.last_updated = action_env.id;
                }
            }
        }
    }

    fn update_pc(&mut self, action_env: ActionEnv, action: &EcsAction) {
        for entity_id in action.pc_positive_iter(action_env.ecs) {
            let entity = action_env.ecs.post_action_entity(entity_id, action);
            if let Some(position) = entity.position() {
                if !entity.current_contains_pc() {
                    // entity is gaining pc
                    let cell = self.get_mut_with_default(position);
                    cell.pc += 1;
                    cell.last_updated = action_env.id;
                }
            }
        }

        for entity_id in action.pc_negative_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            if let Some(position) = entity.position() {
                if entity.contains_pc() {
                    // removing pc from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.pc -= 1;
                    cell.last_updated = action_env.id;
                }
            }
        }
    }

    fn update_moon(&mut self, action_env: ActionEnv, action: &EcsAction) {
        for entity_id in action.moon_positive_iter(action_env.ecs) {
            let entity = action_env.ecs.post_action_entity(entity_id, action);
            if let Some(position) = entity.position() {
                if !entity.current_contains_moon() {
                    // entity is gaining moon
                    let cell = self.get_mut_with_default(position);
                    cell.moon += 1;
                    cell.last_updated = action_env.id;
                }
            }
        }

        for entity_id in action.moon_negative_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            if let Some(position) = entity.position() {
                if entity.contains_moon() {
                    // removing moon from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.moon -= 1;
                    cell.last_updated = action_env.id;
                }
            }
        }
    }

    fn update_opacity(&mut self, action_env: ActionEnv, action: &EcsAction) {
        for (entity_id, new_opacity) in action.opacity_positive_iter(action_env.ecs) {
            let entity = action_env.ecs.post_action_entity(entity_id, action);
            if let Some(position) = entity.position() {
                let current_opacity = entity.current_opacity().unwrap_or(0.0);
                let opacity_increase = new_opacity - current_opacity;
                let cell = self.get_mut_with_default(position);
                cell.opacity += opacity_increase;
                cell.last_updated = action_env.id;
            }
        }

        for entity_id in action.opacity_negative_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            if let Some(position) = entity.position() {
                if let Some(opacity) = entity.opacity() {
                    // removing opacity from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.opacity -= opacity;
                    cell.last_updated = action_env.id;
                }
            }
        }
    }

    fn update_doors(&mut self, action_env: ActionEnv, action: &EcsAction) {
        for (entity_id, _) in action.door_state_positive_iter(action_env.ecs) {
            let entity = action_env.ecs.post_action_entity(entity_id, action);
            if let Some(position) = entity.position() {
                if entity.current_door_state().is_none() {
                    let cell = self.get_mut_with_default(position);
                    cell.doors.insert(entity_id);
                    cell.last_updated = action_env.id;
                }
            }
        }

        for entity_id in action.door_state_negative_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            if let Some(position) = entity.position() {
                if entity.contains_door_state() {
                    // removing door from entity with position
                    let cell = self.get_mut_with_default(position);
                    cell.doors.remove(entity_id);
                    cell.last_updated = action_env.id;
                }
            }
        }
    }

    pub fn update(&mut self, action_env: ActionEnv, action: &EcsAction) {

        for (entity_id, new_position) in action.position_positive_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            // Add and remove tracked components based on the current data stored about the
            // entity, ignoring any component changes in the current action. These will be
            // applied later.
            if let Some(current_position) = entity.position() {
                // the entity is changing position
                self.change_entity_position(entity, current_position, *new_position, action_env.id);
            } else {
                // the entity is gaining a position
                self.add_entity_position(entity, *new_position, action_env.id);
            }
        }

        for entity_id in action.position_negative_iter(action_env.ecs) {
            let entity = action_env.ecs.entity(entity_id);
            if let Some(position) = entity.position() {
                self.remove_entity_position(entity, position, action_env.id);
            }
        }

        self.update_solid(action_env, action);
        self.update_pc(action_env, action);
        self.update_moon(action_env, action);
        self.update_opacity(action_env, action);
        self.update_doors(action_env, action);
    }
}
