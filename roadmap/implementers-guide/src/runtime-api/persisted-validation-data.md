# Persisted Validation Data

Yields the [`PersistedValidationData`](../types/candidate.md#persistedvalidationdata) for the given [`AllyId`](../types/candidate.md#allyid) along with an assumption that should be used if the ally currently occupies a core:

```rust
/// Returns the persisted validation data for the given ally and occupied core assumption.
///
/// Returns `None` if either the ally is not registered or the assumption is `Freed`
/// and the ally already occupies a core.
fn persisted_validation_data(at: Block, AllyId, OccupiedCoreAssumption) -> Option<PersistedValidationData>;
```