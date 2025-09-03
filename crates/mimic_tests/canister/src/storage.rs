///
/// StorageTester
/// Verifies storage snapshot and per-entity breakdown include fixture data.
///
pub struct StorageTester;

impl StorageTester {
    pub fn test() {
        let tests: Vec<(&str, fn())> = vec![("fixture_breakdown", Self::fixture_breakdown)];

        for (name, test_fn) in tests {
            println!("Running test: {name}");
            test_fn();
        }
    }

    fn fixture_breakdown() {
        // Call storage snapshot
        let snap = crate::mimic_snapshot().expect("mimic_storage ok");

        // Should include the FixtureStore with >0 entries in storage_data
        let fixture_store = snap
            .storage_data
            .iter()
            .find(|s| s.path.ends_with("::FixtureStore"))
            .expect("FixtureStore present");
        assert!(fixture_store.entries > 0);

        // entity_storage should include at least one entry for Rarity fixtures
        let rarity = snap
            .entity_storage
            .iter()
            .find(|e| e.store.ends_with("::FixtureStore") && e.path.ends_with("::Rarity"))
            .expect("Rarity entry present in entity_storage");

        assert!(rarity.entries > 0);
        assert!(rarity.memory_bytes > 0);
    }
}
