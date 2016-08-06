use ecs::update_monad::UpdateMonad;

pub enum Control {
    Action(UpdateMonad<()>),
    Quit,
}
