use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice;

use ecs::system::{System, SystemName};

pub type SystemTable = HashMap<SystemName, RefCell<System>>;

#[derive(Debug)]
pub struct SystemQueue {
    systems: SystemTable,
    order: Rc<Vec<SystemName>>,
}

pub struct SystemIter<'a> {
    iter: slice::Iter<'a, SystemName>,
    systems: &'a SystemTable,
}

impl<'a> Iterator for SystemIter<'a> {
    type Item = &'a RefCell<System>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|name| {
            self.systems.get(name).unwrap()
        })
    }
}

impl SystemQueue {
    pub fn new() -> Self {
        SystemQueue {
            systems: HashMap::new(),
            order: Rc::new(Vec::new()),
        }
    }

    pub fn add(&mut self, name: SystemName, system: System) {
        if let Some(_) = self.systems.insert(name, RefCell::new(system)) {
            panic!("System named {:?} already exists.", name);
        }
        let mut order_copy = self.order.to_vec();
        order_copy.push(name);
        self.order = Rc::new(order_copy);
    }

    pub fn get(&self, name: SystemName) -> &RefCell<System> {
        self.systems.get(&name).unwrap()
    }

    pub fn iter<'a>(&'a self) -> SystemIter<'a> {
        SystemIter {
            iter: self.order.iter(),
            systems: &self.systems,
        }
    }
}

macro_rules! system_queue {
    ( $( $name:expr => $system:expr ),* , ) => { system_queue!( $( $name => $system ),* ) };
    ( $( $name:expr => $system:expr ),* ) => {{

        let mut queue = ecs::system_queue::SystemQueue::new();

        $( queue.add($name, $system); )*

        queue
    }};
}
