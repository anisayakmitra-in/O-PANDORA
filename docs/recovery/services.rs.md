# Recovery Document — `services.rs`

## Original path
`crates/pandora-types/src/services.rs`

## Original purpose
Defines service contract traits for all constitutional services. Every
service is replaceable — code depends on traits, never on concrete
implementations. Each trait has exactly one responsibility.

## Public API
- `ServiceId` enum — 14 variants identifying each service
- `ServiceId::as_str()` — string representation
- `Service` trait — base trait (service_id, provider_name, version, health)
- `MemoryService` trait — 6 methods (store, retrieve, forget, search, archive, summarize)
- `ExecutionService` trait — 5 methods (spawn, execute, checkpoint, restore, teardown)
- `PlanningService` trait — 4 methods (plan, dag, retry_plan, topology)
- `GovernanceService` trait — 4 methods (evaluate, audit, score, verify)
- `IdentityService` trait — 4 methods (persist, resurrect, fork, merge)
- `SecurityService` trait — 3 methods (authenticate, authorize, isolate)
- `ProviderService` trait — 9 methods (list_models, health, context_limit, cost,
  latency, invoke, supports_tools, supports_images, supports_reasoning)
- `BenchmarkService` trait — 4 methods (record, query, compare, trend)
- `SchedulerService` trait — 4 methods (schedule, cancel, list, history)
- `TelemetryService` trait — 3 methods (record, query, aggregate)
- `StorageService` trait — 4 methods (read, write, delete, list)
- `CommunicationService` trait — 4 methods (send, broadcast, subscribe, unsubscribe)

## Exported symbols
- `ServiceId` (enum) + its impl blocks
- All 14 traits above

## Dependency relationships
- No internal crate dependencies (self-contained)
- Used by: pandora-services (implementations), pandora-orchestrator (dispatch),
  some harness and evaluator code

## Key algorithms
- `ServiceId::as_str()` — match on all 14 variants
- `Service` trait has a default `health()` implementation returning `Ok(())`
- `ProviderService` has 3 default methods (supports_tools, supports_images,
  supports_reasoning) returning `false`

## Invariants
- All Service traits require `Send + Sync + Debug`
- Each trait inherits from `Service` (not independent)
- Methods return `Result<_, String>` consistently

## Restoration instructions
Replace file. 1 enum + 14 traits. Standard trait patterns with
consistent error types. ServiceId has Display impl via as_str().
