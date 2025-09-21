use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::traits::domain_event::DomainEvent;

pub trait EventListener: Sync + Send {
    fn on_event(&self, event: Arc<dyn DomainEvent>);
}

#[derive(Default)]
pub struct EventBus {
    listeners: RwLock<HashMap<&'static str, Vec<Box<dyn EventListener>>>>,
}

impl EventBus {
    /// Creates a new instance of EventBus with an empty HashMap
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(HashMap::default()),
        }
    }

    /// Adds a new subscriber to a specific topic. Subscribers of the topic are notified of
    /// DomainEvents that are published on that topic
    /// # Arguments
    /// * `topic` - The topic the subscribers will be notified on
    /// * `subscriber` -  The subscriber to be added
    pub fn add_subscriber(&self, topic: &'static str, subscriber: Box<dyn EventListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.entry(topic).or_default().push(subscriber);
    }

    pub fn subscriber_count(&self) -> usize {
        self.listeners.read().unwrap().len()
    }
}

pub trait EventEmitter: Sync + Send {
    /// Publishes a DomainEvent to a topic
    /// # Arguments
    /// * `topic` - The topic the DomainEvent will be published to. Subscribers listening to the topic are notified
    /// * `event` - The DomainEvent to be published
    fn publish(&self, topic: &'static str, event: Arc<dyn DomainEvent>);
}

impl EventEmitter for EventBus {
    fn publish(&self, topic: &'static str, event: Arc<dyn DomainEvent>) {
        if let Some(subs) = self.listeners.read().unwrap().get(topic) {
            for s in subs {
                s.on_event(event.clone())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, atomic::AtomicUsize};

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
                issuer_id: IssuerId::new("test.client".to_string()),
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

    struct AccountListener {
        count: Arc<AtomicUsize>,
    }

    impl AccountListener {
        fn new() -> (Self, Arc<AtomicUsize>) {
            let count = Arc::new(AtomicUsize::default());
            (
                Self {
                    count: count.clone(),
                },
                count,
            )
        }

        fn increment(&self) {
            self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    impl EventListener for AccountListener {
        fn on_event(&self, event: Arc<dyn DomainEvent>) {
            dbg!("Received event");
            let event = event.as_any().downcast_ref::<CreatedAccount>().unwrap();
            dbg!("{:?}", event);

            self.increment();
        }
    }

    #[test]
    fn given_a_listener_and_event_bus_when_publishing_an_event_then_the_listeners_should_receive_it()
     {
        let bus = EventBus::new();
        let (sub1, count1) = AccountListener::new();
        let (sub2, count2) = AccountListener::new();

        bus.add_subscriber("account-created", Box::new(sub1));
        bus.add_subscriber("account-updated", Box::new(sub2));

        assert_eq!(bus.subscriber_count(), 2);

        let event = CreatedAccount::new(Uuid::new_v4());
        let event2 = CreatedAccount::new(Uuid::new_v4());
        bus.publish("account-created", Arc::new(event));
        bus.publish("account-created", Arc::new(event2));

        assert_eq!(count1.load(std::sync::atomic::Ordering::SeqCst), 2);
        assert_eq!(count2.load(std::sync::atomic::Ordering::SeqCst), 0);
    }
}
