use game::{SpatialHashTable, ActionArgs, Result};
use ecs::{EcsCtx, EcsAction};

pub struct Reaction {
    pub action: ActionArgs,
    pub delay: u64,
}

#[derive(Clone, Copy)]
pub struct RuleEnv<'a> {
    pub ecs: &'a EcsCtx,
    pub spatial_hash: &'a SpatialHashTable,
}

pub struct RuleResolution {
    reactions: Vec<Reaction>,
    accept: bool,
}

pub trait Rule {
    fn check(&self, env: RuleEnv, action: &EcsAction, resolution: &mut RuleResolution) -> Result<()>;
}

impl Reaction {
    pub fn new(action: ActionArgs, delay: u64) -> Self {
        Reaction {
            action: action,
            delay: delay,
        }
    }
}

impl RuleResolution {
    pub fn new() -> Self {
        RuleResolution {
            reactions: Vec::new(),
            accept: true,
        }
    }

    pub fn reset(&mut self) {
        self.reactions.clear();
        self.accept = true;
    }

    pub fn reject(&mut self) {
        self.reactions.clear();
        self.accept = false;
    }

    pub fn is_accept(&self) -> bool {
        self.accept
    }

    pub fn is_reject(&self) -> bool {
        !self.accept
    }

    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }
}
