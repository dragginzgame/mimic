use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    user_index(builder);
}

// user_index
pub fn user_index(builder: &mut ActorBuilder) {
    let q = quote! {

        // user_index
        #[::mimic::ic::query]
        async fn user_index() -> Result<UserIndex, ::mimic::api::Error> {
            allow_any(vec![Auth::Controller]).await?;

            Ok(UserIndexManager::get())
        }

        // get_caller
        // no auth needed as it's just looking up the current caller
        #[::mimic::ic::query]
        fn get_caller() -> Result<User, ::mimic::api::Error> {
            let user = UserIndexManager::try_get_user(caller())?;

            Ok(user)
        }

        // get_user
        // look up any user by principal, requires an auth check
        #[::mimic::ic::query]
        async fn get_user(id: Principal) -> Result<User, ::mimic::api::Error> {
            if id != caller() {
                allow_any(vec![Auth::Controller]).await?;
            }

            let user = UserIndexManager::try_get_user(id)?;

            Ok(user)
        }

        // register_caller
        #[::mimic::ic::update(guard = "guard_update")]
        async fn register_caller() -> Result<User, ::mimic::api::Error> {
            let user = register(caller()).await?;

            Ok(user)
        }

        // register_principal
        // register ANY principal, requires controller or parent
        #[::mimic::ic::update]
        async fn register_principal(id: Principal) -> Result<User, ::mimic::api::Error> {
            allow_any(vec![
                Auth::Controller,
                Auth::SameCanister,
            ]).await?;

            let user = register(id).await?;

            Ok(user)
        }

        // add_role
        #[::mimic::ic::update]
        async fn add_role(id: Principal, role: String) -> Result<(), ::mimic::api::Error> {
            allow_any(vec![
                Auth::Parent,
                Auth::Controller,
            ]).await?;

            UserIndexManager::add_role(id, role)?;

            Ok(())
        }

        // remove_role
        #[::mimic::ic::update(guard = "guard_update")]
        async fn remove_role(id: Principal, role: String) -> Result<(), ::mimic::api::Error> {
            allow_any(vec![
                Auth::Parent,
                Auth::Controller,
            ]).await?;

            UserIndexManager::remove_role(id, role).map_err(::mimic::api::Error::from)?;

            Ok(())
        }

        // guard_permission
        // endpoint only works on the User canister
        // has to return api::Error as it's called by the api crate
        #[::mimic::ic::query]
        pub async fn guard_permission(id: ::candid::Principal, permission: String) -> Result<(), ::mimic::api::Error> {
            let user = UserIndexManager::try_get_user(id)?;

            // return Ok if any role has the permission, otherwise return an error
            if user
                .roles
                .iter()
                .any(|role| ::mimic::core::schema::AuthService::role_has_permission(role, &permission))
            {
                Ok(())
            } else {
                Err(::mimic::api::auth::Error::NotPermitted {
                    id,
                    permission,
                }).map_err(::mimic::api::Error::from)
            }
        }
    };

    builder.extend_actor(q);
}
