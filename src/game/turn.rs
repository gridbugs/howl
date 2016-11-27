use ecs::EcsCtx;

#[derive(Clone, Copy)]
pub struct Turn<'a> {
    pub ecs: &'a EcsCtx,
    pub id: u64,
}

impl<'a> Turn<'a> {
    pub fn new(ecs: &'a EcsCtx, id: u64) -> Self {
        Turn {
            ecs: ecs,
            id: id,
        }
    }
}
