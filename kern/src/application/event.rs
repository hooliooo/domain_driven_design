use std::sync::Arc;

use crate::building_blocks::domain_event::DynDomainEvent;

/// The EventPublisher sends the Domain Event to the local message queu
pub trait EventPublisher: Send + Sync {
    /// Publishes the domain event to the topic
    /// # Arguments
    /// * `topic` - The topic the domain event will be published to
    /// * `event` - The domain event to be published
    fn publish(&self, topic: &'static str, event: Arc<dyn DynDomainEvent>);
}

/// The EventHandler acts on the Domain Event it receives from the EventBus
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Handles the domain event
    /// # Arguments
    /// * `event` - The domain event to handle
    async fn handle(&self, event: Arc<dyn DynDomainEvent>);
}

/// The EventBus is the orchestrator between the EventHandler and EventPublisher
pub trait EventBus: Send + Sync {
    /// Registers a handler to a specific topic so the handler can act on that topic whenever an
    /// event is published there
    /// # Arguments
    /// * `topic` - The topic the handler will be registered to
    /// * `handler` - The handler that will act on events sent to the topic
    fn register_handler(&self, topic: &'static str, handler: Box<dyn EventHandler>);
}
