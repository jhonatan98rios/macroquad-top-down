use std::any::Any;
use std::collections::HashMap;

pub type Callback = Box<dyn FnMut(&mut dyn Any, &EventPayload)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Damage,
    Heal,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum EventPayload {
    Damage { amount: u32 },
    Heal { amount: u32 },
    Custom(String),
}

pub struct EventBus {
    subscribers: HashMap<EventType, Vec<Callback>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe<T: 'static, F>(&mut self, event: EventType, mut handler: F)
    where
        F: FnMut(&mut T, &EventPayload) + 'static,
    {
        let cb: Callback = Box::new(move |target, payload| {
            if let Some(t) = target.downcast_mut::<T>() {
                handler(t, payload);
            }
        });

        self.subscribers.entry(event).or_default().push(cb);
    }

    pub fn emit(&mut self, event: &EventType, target: &mut dyn Any, payload: &EventPayload) {
        if let Some(list) = self.subscribers.get_mut(event) {
            for cb in list {
                cb(target, payload);
            }
        }
    }
}
