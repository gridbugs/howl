use ecs::message::Message;
use ecs::message_queue::MessageQueue;
use ecs::entity::EntityTable;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice;

use ecs::system::{System, SystemName};

pub type SystemTable<'a> = HashMap<SystemName, RefCell<System<'a>>>;

#[derive(Debug)]
pub struct SystemQueue<'a> {
    systems: SystemTable<'a>,
    order: Rc<Vec<SystemName>>,
}

pub struct SystemIter<'a> {
    iter: slice::Iter<'a, SystemName>,
    systems: &'a SystemTable<'a>,
}

impl<'a> Iterator for SystemIter<'a> {
    type Item = &'a RefCell<System<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|name| {
            self.systems.get(name).unwrap()
        })
    }
}

impl<'a> SystemQueue<'a> {
    pub fn new() -> Self {
        SystemQueue {
            systems: HashMap::new(),
            order: Rc::new(Vec::new()),
        }
    }

    pub fn add<'b>(&'b mut self, name: SystemName, system: System<'a>) {
        if let Some(_) = self.systems.insert(name, RefCell::new(system)) {
            panic!("System named {:?} already exists.", name);
        }
        let mut order_copy = self.order.to_vec();
        order_copy.push(name);
        self.order = Rc::new(order_copy);
    }

    pub fn get(&'a self, name: SystemName) -> &'a RefCell<System<'a>> {
        self.systems.get(&name).unwrap()
    }

    pub fn iter(&'a self) -> SystemIter<'a> {
        SystemIter {
            iter: self.order.iter(),
            systems: &self.systems,
        }
    }

    pub fn process_message(&'a self, message: &mut Message,
                           entities: &mut EntityTable,
                           systems: &SystemQueue,
                           message_queue: &mut MessageQueue)
    {
        for system in self.iter() {
            system.borrow_mut().process_message(message, entities, systems, message_queue);
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
