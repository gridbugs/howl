use ecs::update::UpdateSummary;
use ecs::entity::EntityTable;

use std::boxed::FnBox;

pub type Action = UpdateMonad<()>;

pub struct UpdateMonad<A> (
    Box<Fn(&mut UpdateSummary, &mut EntityTable) -> A>
);

impl<A: 'static + Copy> UpdateMonad<A> {

    pub fn ret(a: A) -> Self {
        UpdateMonad(Box::new(move |_, _| { a }))
    }

    pub fn new<F>(f: F) -> Self
        where F: 'static + Fn(&mut UpdateSummary, &mut EntityTable) -> A
    {
        UpdateMonad(Box::new(f))
    }

    pub fn bind<B: 'static + Copy, F>(self, f: F) -> UpdateMonad<B>
        where F: 'static + Fn(A) -> UpdateMonad<B>
    {
        UpdateMonad(Box::new(move |summary, entities| {
            let value : A = self.0(summary, entities);

            let next : UpdateMonad<B> = f(value);

            next.0(summary, entities)
        }))
    }

    pub fn apply(&self, entities: &mut EntityTable) -> UpdateSummary {
        let mut summary = UpdateSummary::new();
        self.0(&mut summary, entities);
        summary
    }
}

pub struct M<A> (
    Box<FnBox(i32) -> A>

);

impl<A: 'static + Copy> M<A> {

    pub fn new<F>(f: F) -> Self
        where F: 'static + FnOnce(i32) -> A
    {
        M(Box::new(f))
    }

    pub fn ret(a: A) -> Self {
        M(Box::new(move |_| { a }))
    }
/*
    pub fn bind<B: 'static + Copy, F>(self, f: F) -> M<B>
        where F: 'static + FnOnce(A) -> M<B>
    {

        M(Box::new(move |entities| {
            let current : A = self.0(entities);

            let rest : M<B> = f(current);

            rest.0()
        }))
    }
    */
}
