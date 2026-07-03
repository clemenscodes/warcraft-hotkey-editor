//! Tactical and strategic Domain-Driven Design building blocks.
//!
//! This crate defines the vocabulary of a domain model as traits, so that "this
//! is an aggregate root", "this is a domain event", "this type is infrastructure"
//! become facts the compiler checks rather than conventions a reader must
//! remember. It is pure: no async runtime, no browser, no framework — it depends
//! on nothing and runs under native `cargo test`. It is itself a
//! [`SharedKernel`]: the language every bounded context speaks.
//!
//! Grouped by concern:
//!
//! - **Building blocks:** [`ValueObject`], [`Entity`], [`Identifier`],
//!   [`AggregateRoot`], [`Factory`].
//! - **Domain logic:** [`DomainService`], [`Specification`], [`Policy`].
//! - **Application / CQRS:** [`Command`], [`CommandHandler`], [`Query`],
//!   [`QueryHandler`], [`ApplicationService`], [`Service`].
//! - **Events:** [`DomainEvent`], [`EventHandler`], [`EventBus`], [`EventStore`],
//!   [`Projection`], [`ReadModel`].
//! - **Persistence:** [`Repository`], [`UnitOfWork`].
//! - **Hexagonal:** [`Port`], [`DrivingPort`], [`DrivenPort`], [`Adapter`].
//! - **Layers:** [`Layer`], [`Layered`], [`DomainLayer`], [`ApplicationLayer`],
//!   [`InfrastructureLayer`], [`PresentationLayer`].
//! - **Strategic:** [`BoundedContext`], [`SharedKernel`], [`AntiCorruptionLayer`],
//!   [`Saga`].
//!
//! # Three tiers of enforcement
//!
//! - **Structural** traits oblige real behaviour, so they enforce architecture:
//!   [`Repository`], [`Command`], [`Service`], [`Specification`].
//! - **Intent markers** enforce little or nothing and exist to make a type's role
//!   legible and greppable: [`DomainService`], [`DomainEvent`], [`ReadModel`],
//!   the strategic markers.
//! - **Layer membership** is a single-valued associated type ([`Layered`]), which
//!   makes the four layers *mutually exclusive*: a type carries exactly one
//!   `Layered` impl, so a second layer is a conflicting implementation that does
//!   not compile. This is how you reject wrong-layer types without a negative
//!   trait bound — Rust has none, by design. Role markers pin a layer through it:
//!   [`AggregateRoot`] requires [`DomainLayer`], [`ApplicationService`] requires
//!   [`ApplicationLayer`], so one type can never be both.
//!
//! What none of these enforce is the dependency rule itself — that an inner layer
//! never *names* an outer one. Only the crate graph does that, by making the
//! dependency physically absent. Markers label and guard seams; crates are the
//! wall.

mod adapter;
mod aggregate_root;
mod anti_corruption_layer;
mod application_service;
mod bounded_context;
mod command;
mod command_handler;
mod domain_event;
mod domain_service;
mod entity;
mod event_bus;
mod event_handler;
mod event_store;
mod factory;
mod identifier;
mod layer;
mod policy;
mod projection;
mod query;
mod query_handler;
mod read_model;
mod repository;
mod saga;
mod service;
mod shared_kernel;
mod specification;
mod unit_of_work;
mod value_object;

pub use adapter::Adapter;
pub use aggregate_root::AggregateRoot;
pub use anti_corruption_layer::AntiCorruptionLayer;
pub use application_service::ApplicationService;
pub use bounded_context::BoundedContext;
pub use command::Command;
pub use command_handler::CommandHandler;
pub use domain_event::DomainEvent;
pub use domain_service::DomainService;
pub use entity::Entity;
pub use event_bus::EventBus;
pub use event_handler::EventHandler;
pub use event_store::EventStore;
pub use factory::Factory;
pub use identifier::Identifier;
pub use layer::ApplicationLayer;
pub use layer::DomainLayer;
pub use layer::InfrastructureLayer;
pub use layer::Layer;
pub use layer::Layered;
pub use layer::PresentationLayer;
pub use policy::Policy;
pub use projection::Projection;
pub use query::Query;
pub use query_handler::QueryHandler;
pub use read_model::ReadModel;
pub use repository::Repository;
pub use saga::Saga;
pub use service::Service;
pub use shared_kernel::SharedKernel;
pub use specification::And;
pub use specification::Not;
pub use specification::Or;
pub use specification::Specification;
pub use unit_of_work::UnitOfWork;
pub use value_object::ValueObject;
