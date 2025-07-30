use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::traits::domain_event::DomainEvent;

pub trait EventListener: Sync + Send {
    fn on_event(&self, event: Arc<dyn DomainEvent>);
}

#[derive(Default)]
struct EventBus {
    listeners: RwLock<HashMap<&'static str, Vec<Box<dyn EventListener>>>>,
}

impl EventBus {
    /// Creates a new instance of EventBus with an empty HashMap
    fn new() -> Self {
        Self {
            listeners: RwLock::new(HashMap::default()),
        }
    }

    /// Adds a new subscriber to a specific topic. Subscribers of the topic are notified of
    /// DomainEvents that are published on that topic
    /// # Arguments
    /// * `topic` - The topic the subscribers will be notified on
    /// * `subscriber` -  The subscriber to be added
    fn add_subscriber(&mut self, topic: &'static str, subscriber: Box<dyn EventListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.entry(topic).or_insert_with(|| vec![subscriber]);
    }
}

pub trait EventEmitter: Sync + Send {
    /// Publishes a DomainEvent to a topic
    /// # Arguments
    /// * `topic` - The topic the DomainEvent will be published to. Subscribers listening to the\
    /// topic are notified
    /// * `event` - The DomainEvent to be published
    fn publish(&self, topic: &'static str, event: Arc<dyn DomainEvent>);
}

impl EventEmitter for EventBus {
    fn publish(&self, topic: &'static str, event: Arc<dyn DomainEvent>) {
        if let Some(subs) = self.listeners.read().unwrap().get(topic) {
            #[cfg(test)]
            {
                dbg!("Publishing event");
            }

            for s in subs {
                s.on_event(event.clone())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    use crate::{
        structs::ids::{CommandId, EventId, IssuerId},
        traits::domain_event::DomainEvent,
    };

    use super::{EventBus, EventEmitter, EventListener};

    #[derive(Debug)]
    pub struct CreatedAccount {
        command_id: CommandId,
        environment: crate::enums::environment::Environment,
        issuer_id: IssuerId,
        event_id: EventId,
        issued_at: DateTime<Utc>,
    }

    impl CreatedAccount {
        pub fn new(command_id: uuid::Uuid) -> Self {
            Self {
                command_id: CommandId::new(command_id),
                environment: crate::enums::environment::Environment::Development,
                event_id: EventId::new_random(),
                issuer_id: IssuerId::new(Uuid::new_v4()),
                issued_at: Utc::now(),
            }
        }
    }

    impl DomainEvent for CreatedAccount {
        fn command_id(&self) -> &CommandId {
            &self.command_id
        }

        fn event_id(&self) -> &EventId {
            &self.event_id
        }

        fn issuer_id(&self) -> &IssuerId {
            &self.issuer_id
        }

        fn issued_at(&self) -> &chrono::DateTime<chrono::Utc> {
            &self.issued_at
        }

        fn environment(&self) -> &crate::enums::environment::Environment {
            &self.environment
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    struct AccountListener;

    impl EventListener for AccountListener {
        fn on_event(&self, event: Arc<dyn DomainEvent>) {
            dbg!("Received event");
            let event = event.as_any().downcast_ref::<CreatedAccount>().unwrap();
            dbg!("{:?}", event);
        }
    }

    #[test]
    fn given_a_listener_and_event_bus_when_publishing_an_event_then_the_listeners_should_receive_it()
     {
        let mut bus = EventBus::new();
        bus.add_subscriber("account-created", Box::new(AccountListener));
        let event = CreatedAccount::new(Uuid::new_v4());
        bus.publish("account-created", Arc::new(event));
    }
}
