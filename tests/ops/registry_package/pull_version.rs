use cargo_update::ops::{RegistryPackage, RegistryTree, Registry};


/// A package deleted from the registry has nothing to update to, but mustn't blow up.
///
/// As left by `update_index()`: polled, HTTP 404, no entry at all.
#[test]
fn deleted_from_registry() {
    assert!(!check(&Registry::Sparse(Default::default())));
}

/// A package whose entry exists but lists no versions is still in the registry, just not updatable.
#[test]
fn no_versions() {
    assert!(check(&Registry::Sparse([("gone".to_string(), vec![])].into())));
}

fn check(registry: &Registry) -> bool {
    let mut package = RegistryPackage::parse("gone 0.1.0 (registry+https://github.com/rust-lang/crates.io-index)", vec!["gone".to_string()]).unwrap();

    let in_registry = package.pull_version(&RegistryTree::Sparse, registry, None, None);

    assert_eq!(package.newest_version, None);
    assert_eq!(package.update_to_version(), None);
    assert!(!package.needs_update(None, None, false));
    assert!(!package.needs_update(None, None, true));
    in_registry
}
