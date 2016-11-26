use ecs::EcsCtx;

#[derive(Clone, Copy)]
pub struct Turn<'a> {
    pub ecs: &'a EcsCtx,
    pub id: u64,
}
