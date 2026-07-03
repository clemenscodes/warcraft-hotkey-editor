use std::cell::RefCell;
use std::rc::Rc;

use ddd::Adapter;
use ddd::AggregateRoot;
use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::Command;
use ddd::DomainLayer;
use ddd::Entity;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::Repository;
use ddd::Service;
use ddd::Specification;
use ddd::ValueObject;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Counter {
    value: u32,
}

impl Counter {
    fn value(&self) -> u32 {
        self.value
    }

    fn set_value(&mut self, value: u32) {
        self.value = value;
    }
}

impl Layered for Counter {
    type Layer = DomainLayer;
}

impl AggregateRoot for Counter {}

#[derive(Clone, Debug, Default)]
struct InMemoryCounterRepository {
    stored: Rc<RefCell<Option<Counter>>>,
}

impl Layered for InMemoryCounterRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for InMemoryCounterRepository {}

impl Repository<Counter> for InMemoryCounterRepository {
    fn load(&self) -> Option<Counter> {
        let stored = self.stored.borrow();
        *stored
    }

    fn save(&self, aggregate: &Counter) {
        let snapshot = *aggregate;
        let mut stored = self.stored.borrow_mut();
        *stored = Some(snapshot);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Increment {
    amount: u32,
}

impl Layered for Increment {
    type Layer = ApplicationLayer;
}

impl Command<Counter> for Increment {
    type Outcome = u32;

    fn execute(self, aggregate: &mut Counter) -> u32 {
        let current_value = aggregate.value();
        let incremented_value = current_value + self.amount;
        aggregate.set_value(incremented_value);
        aggregate.value()
    }
}

#[derive(Clone, Debug, Default)]
struct CounterService {
    aggregate: Rc<RefCell<Counter>>,
    repository: InMemoryCounterRepository,
}

impl Layered for CounterService {
    type Layer = ApplicationLayer;
}

impl ApplicationService for CounterService {}

impl Service<Counter> for CounterService {
    type Repository = InMemoryCounterRepository;

    fn repository(&self) -> Self::Repository {
        self.repository.clone()
    }

    fn snapshot(&self) -> Counter {
        let aggregate = self.aggregate.borrow();
        *aggregate
    }

    fn replace(&self, aggregate: Counter) {
        let mut current = self.aggregate.borrow_mut();
        *current = aggregate;
    }
}

fn accept_only_domain_layer<Type>()
where
    Type: Layered<Layer = DomainLayer>,
{
}

#[test]
fn dispatch_applies_command_and_writes_through_to_the_repository() {
    let service = CounterService::default();

    let increment = Increment { amount: 3 };
    let outcome = service.dispatch(increment);

    assert_eq!(outcome, 3);

    let live_aggregate = service.snapshot();
    assert_eq!(live_aggregate.value(), 3);

    let persisted_aggregate = service.repository().load();
    let expected_aggregate = Counter { value: 3 };
    let expected_persisted = Some(expected_aggregate);
    assert_eq!(persisted_aggregate, expected_persisted);
}

#[test]
fn commit_is_the_write_through_path_for_named_mutations() {
    let service = CounterService::default();

    service.commit(|counter| {
        let recomputed_value = counter.value() + 5;
        counter.set_value(recomputed_value);
    });

    let persisted_aggregate = service.repository().load();
    let expected_aggregate = Counter { value: 5 };
    let expected_persisted = Some(expected_aggregate);
    assert_eq!(persisted_aggregate, expected_persisted);
}

#[test]
fn a_domain_layer_bound_accepts_a_domain_type() {
    // Counter is tagged `DomainLayer`; `CounterService` (application layer) would
    // be rejected here — the compile_fail doctest on `Layered` shows why.
    accept_only_domain_layer::<Counter>();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct GridCoordinate {
    column: u32,
    row: u32,
}

impl Layered for GridCoordinate {
    type Layer = DomainLayer;
}

impl ValueObject for GridCoordinate {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct StoredHotkey {
    slot: u32,
    key: u32,
}

impl Layered for StoredHotkey {
    type Layer = DomainLayer;
}

impl Entity for StoredHotkey {
    type Identity = u32;

    fn identity(&self) -> &Self::Identity {
        &self.slot
    }
}

#[test]
fn value_objects_compare_by_value_and_entities_by_identity() {
    let first_coordinate = GridCoordinate { column: 1, row: 2 };
    let second_coordinate = GridCoordinate { column: 1, row: 2 };
    assert_eq!(first_coordinate, second_coordinate);

    let original_binding = StoredHotkey { slot: 7, key: 81 };
    let rebound_binding = StoredHotkey { slot: 7, key: 87 };
    let original_identity = original_binding.identity();
    let rebound_identity = rebound_binding.identity();
    assert_eq!(original_identity, rebound_identity);
    assert_ne!(original_binding, rebound_binding);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct AtLeast {
    threshold: u32,
}

impl Layered for AtLeast {
    type Layer = DomainLayer;
}

impl Specification<Counter> for AtLeast {
    fn is_satisfied_by(&self, candidate: &Counter) -> bool {
        candidate.value() >= self.threshold
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct AtMost {
    ceiling: u32,
}

impl Layered for AtMost {
    type Layer = DomainLayer;
}

impl Specification<Counter> for AtMost {
    fn is_satisfied_by(&self, candidate: &Counter) -> bool {
        candidate.value() <= self.ceiling
    }
}

#[test]
fn specifications_compose_with_boolean_combinators() {
    let five = Counter { value: 5 };

    let at_least_three = AtLeast { threshold: 3 };
    let at_most_seven = AtMost { ceiling: 7 };
    let within_range = at_least_three.and(at_most_seven);
    assert!(within_range.is_satisfied_by(&five));

    let at_least_ten = AtLeast { threshold: 10 };
    let below_ten = at_least_ten.not();
    assert!(below_ten.is_satisfied_by(&five));

    let at_least_nine = AtLeast { threshold: 9 };
    let at_most_one = AtMost { ceiling: 1 };
    let at_the_extremes = at_least_nine.or(at_most_one);
    assert!(!at_the_extremes.is_satisfied_by(&five));
}
