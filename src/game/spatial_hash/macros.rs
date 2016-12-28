macro_rules! update_count {
    ($fn_name: ident, $component: ident, $positive_iter: ident, $negative_iter: ident,
     $contains_component: ident, $current_contains_component: ident) => {
        fn $fn_name(&mut self, action_env: ActionEnv, action: &EcsAction) {
            for entity_id in action.$positive_iter(action_env.ecs) {
                let entity = action_env.ecs.post_action_entity(entity_id, action);
                if let Some(position) = entity.position() {
                    if !entity.$current_contains_component() {
                        let cell = self.get_mut(position);
                        cell.$component += 1;
                        cell.last_updated = action_env.id;
                    }
                }
            }

            for entity_id in action.$negative_iter(action_env.ecs) {
                let entity = action_env.ecs.entity(entity_id);
                if let Some(position) = entity.position() {
                    if entity.$contains_component() {
                        let cell = self.get_mut(position);
                        cell.$component -= 1;
                        cell.last_updated = action_env.id;
                    }
                }
            }
        }
    }
}

macro_rules! update_set {
    ($fn_name: ident, $component: ident, $positive_iter: ident, $negative_iter: ident,
     $contains_component: ident, $current_contains_component: ident) => {
        fn $fn_name(&mut self, action_env: ActionEnv, action: &EcsAction) {
            for entity_id in action.$positive_iter(action_env.ecs) {
                let entity = action_env.ecs.post_action_entity(entity_id, action);
                if let Some(position) = entity.position() {
                    if !entity.$current_contains_component() {
                        let cell = self.get_mut(position);
                        cell.$component.insert(entity_id);
                        cell.last_updated = action_env.id;
                    }
                }
            }
            for entity_id in action.$negative_iter(action_env.ecs) {
                let entity = action_env.ecs.entity(entity_id);
                if let Some(position) = entity.position() {
                    if entity.$contains_component() {
                        let cell = self.get_mut(position);
                        cell.$component.remove(entity_id);
                        cell.last_updated = action_env.id;
                    }
                }
            }
        }
    }
}

macro_rules! update_set_typed {
    ($fn_name: ident, $component: ident, $positive_iter: ident, $negative_iter: ident,
     $contains_component: ident, $current_component: ident) => {
        fn $fn_name(&mut self, action_env: ActionEnv, action: &EcsAction) {
            for (entity_id, _) in action.$positive_iter(action_env.ecs) {
                let entity = action_env.ecs.post_action_entity(entity_id, action);
                if let Some(position) = entity.position() {
                    if entity.$current_component().is_none() {
                        let cell = self.get_mut(position);
                        cell.$component.insert(entity_id);
                        cell.last_updated = action_env.id;
                    }
                }
            }
            for entity_id in action.$negative_iter(action_env.ecs) {
                let entity = action_env.ecs.entity(entity_id);
                if let Some(position) = entity.position() {
                    if entity.$contains_component() {
                        let cell = self.get_mut(position);
                        cell.$component.remove(entity_id);
                        cell.last_updated = action_env.id;
                    }
                }
            }
        }
    }
}

macro_rules! update_sum {
    ($fn_name: ident, $component: ident, $current_component: ident, $positive_iter: ident, $negative_iter: ident, $default: expr) => {
        fn $fn_name(&mut self, action_env: ActionEnv, action: &EcsAction) {
            for (entity_id, new) in action.$positive_iter(action_env.ecs) {
                let entity = action_env.ecs.post_action_entity(entity_id, action);
                if let Some(position) = entity.position() {
                    let current = entity.$current_component().unwrap_or($default);
                    let increase = new - current;
                    let cell = self.get_mut(position);
                    cell.$component += increase;
                    cell.last_updated = action_env.id;
                }
            }
            for entity_id in action.$negative_iter(action_env.ecs) {
                let entity = action_env.ecs.entity(entity_id);
                if let Some(position) = entity.position() {
                    if let Some(value) = entity.$component() {
                        let cell = self.get_mut(position);
                        cell.$component -= value;
                        cell.last_updated = action_env.id;
                    }
                }
            }
        }
    }
}
