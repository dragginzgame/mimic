use super::get_schema;
use derive_more::Deref;
use orm_schema::node::Role;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

///
/// AuthService
/// public wrapper for all authentication state
///

pub struct AuthService {}

impl AuthService {
    // role_has_permission
    // Checks if a given role has the specified permission
    #[must_use]
    pub fn role_has_permission(role: &str, permission: &str) -> bool {
        ROLE_PERMISSION_MAP.get(role).map_or(false, |permissions| {
            permissions.contains(&String::from(permission))
        })
    }
}

///
/// RolePermissionMap
///

pub static ROLE_PERMISSION_MAP: LazyLock<RolePermissionMap> =
    LazyLock::new(RolePermissionMap::init);

#[derive(Clone, Debug, Deref)]
pub struct RolePermissionMap(HashMap<String, HashSet<String>>);

impl RolePermissionMap {
    // init
    #[must_use]
    pub fn init() -> Self {
        let mut map = HashMap::new();

        // role by role
        for role in get_schema().unwrap().get_node_values::<Role>() {
            let permissions = Self::collect_permissions(role);
            map.insert(role.def.path(), permissions);
        }

        // ::ic::println!("{map:?}");

        Self(map)
    }

    // collect_permissions
    // Recursively collect permissions from children roles
    fn collect_permissions(parent: &Role) -> HashSet<String> {
        let mut permissions = HashSet::new();
        Self::collect_child_permissions(parent, &mut permissions);

        permissions.into_iter().collect()
    }

    // collect_child_permissions
    // Helper function to recursively collect permissions
    fn collect_child_permissions(role: &Role, permissions: &mut HashSet<String>) {
        // Add current role's permissions
        for p in &role.permissions {
            permissions.insert(p.clone());
        }

        // Recurse into children roles
        for child in get_schema().unwrap().get_node_values::<Role>() {
            if let Some(parent_path) = &child.parent {
                if parent_path == &role.def.path() {
                    Self::collect_child_permissions(child, permissions);
                }
            }
        }
    }
}
