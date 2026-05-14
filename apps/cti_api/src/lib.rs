use std::sync::Arc;

use actix_web::web;

use kernel::source::application::create_source::create_source_command_handler::CreateSourceCommandHandler;
use kernel::source::application::create_source::source_creator::SourceCreator;
use kernel::source::application::delete_source::delete_source_command_handler::DeleteSourceCommandHandler;
use kernel::source::application::delete_source::source_deleter::SourceDeleter;
use kernel::source::application::find_source::find_source_query_handler::FindSourceQueryHandler;
use kernel::source::application::find_source::source_finder::SourceFinder;
use kernel::source::application::update_source::source_updater::SourceUpdater;
use kernel::source::application::update_source::update_source_command_handler::UpdateSourceCommandHandler;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use kernel::source::infrastructure::persistence::in_memory::in_memory_source_repository::InMemorySourceRepository;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::domain::query_bus::QueryBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;

pub mod health;
pub mod source;

/// Shared application state injected into every Actix-Web request handler.
pub struct AppState {
    pub command_bus: Arc<dyn CommandBus>,
    pub query_bus: Arc<dyn QueryBus>,
}

/// Wires all repositories, services and buses together and returns the shared
/// application state. Each call produces an isolated in-memory store, making
/// this function safe to call once per test.
pub fn build_state() -> web::Data<AppState> {
    let repo: Arc<dyn SourceRepository> = Arc::new(InMemorySourceRepository::new());
    let event_bus: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    let creator = SourceCreator::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let create_handler = CreateSourceCommandHandler::new(creator);

    let finder = SourceFinder::new(Arc::clone(&repo));
    let find_handler = FindSourceQueryHandler::new(finder);

    let updater = SourceUpdater::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let update_handler = UpdateSourceCommandHandler::new(updater);

    let deleter = SourceDeleter::new(Arc::clone(&repo), Arc::clone(&event_bus));
    let delete_handler = DeleteSourceCommandHandler::new(deleter);

    let mut command_bus = InMemoryCommandBus::new();
    command_bus
        .register(create_handler)
        .expect("Failed to register CreateSourceCommandHandler");
    command_bus
        .register(update_handler)
        .expect("Failed to register UpdateSourceCommandHandler");
    command_bus
        .register(delete_handler)
        .expect("Failed to register DeleteSourceCommandHandler");
    let command_bus: Arc<dyn CommandBus> = Arc::new(command_bus);

    let mut query_bus = InMemoryQueryBus::new();
    query_bus
        .register(find_handler)
        .expect("Failed to register FindSourceQueryHandler");
    let query_bus: Arc<dyn QueryBus> = Arc::new(query_bus);

    web::Data::new(AppState { command_bus, query_bus })
}

/// Registers all HTTP routes onto an Actix-Web [`ServiceConfig`].
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::get::handler)
        .service(source::controllers::post::handler)
        .service(source::controllers::get::handler)
        .service(source::controllers::put::handler)
        .service(source::controllers::delete::handler);
}
