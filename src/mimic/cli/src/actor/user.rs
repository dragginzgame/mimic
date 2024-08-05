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
        async fn user_index() -> Result<UserIndex, Error> {
            guard(vec![Guard::Controller]).await.map_err(::mimic::Error::from)?;

            Ok(::mimic::api::state::user_index())
        }

        // get_caller
        // no auth needed as it's just looking up the current caller
        #[::mimic::ic::query]
        fn get_caller() -> Result<User, Error> {
            let user = UserIndexManager::try_get_user(caller()).map_err(::mimic::Error::from)?;

            Ok(user)
        }

        // get_user
        // look up any user by principal, requires an auth check
        #[::mimic::ic::query]
        async fn get_user(id: Principal) -> Result<User, Error> {
            if id != caller() {
                guard(vec![Guard::Controller]).await.map_err(::mimic::Error::from)?;
            }

            let user = UserIndexManager::try_get_user(id).map_err(::mimic::Error::from)?;

            Ok(user)
        }

        // register_caller
        #[::mimic::ic::update]
        async fn register_caller() -> Result<User, Error> {
            let user = register(caller()).await?;

            Ok(user)
        }

        // register_principal
        // register ANY principal, requires controller or parent
        #[::mimic::ic::update]
        async fn register_principal(id: Principal) -> Result<User, Error> {
            guard(vec![
                Guard::This,
                Guard::Controller,
            ])
            .await.map_err(::mimic::Error::from)?;

            let user = register(id).await.map_err(::mimic::Error::from)?;

            Ok(user)
        }

        // add_role
        #[::mimic::ic::update]
        async fn add_role(id: Principal, role: String) -> Result<(), Error> {
            guard(vec![
                Guard::Parent,
                Guard::Controller,
            ])
            .await.map_err(::mimic::Error::from)?;

            UserIndexManager::add_role(id, role).map_err(::mimic::Error::from)?;

            Ok(())
        }

        // remove_role
        #[::mimic::ic::update]
        async fn remove_role(id: Principal, role: String) -> Result<(), Error> {
            guard(vec![
                Guard::Parent,
                Guard::Controller,
            ])
            .await.map_err(::mimic::Error::from)?;

            UserIndexManager::remove_role(id, role).map_err(::mimic::Error::from)?;

            Ok(())
        }

        // guard_permission
        // endpoint only works on the User canister
        #[::mimic::ic::query]
        pub async fn guard_permission(id: Principal, permission: String) -> Result<(), Error> {
            let user = UserIndexManager::try_get_user(id).map_err(::mimic::Error::from)?;

            // return Ok if any role has the permission, otherwise return an error
            if user
                .roles
                .iter()
                .any(|role| ::mimic::core::schema::AuthService::role_has_permission(role, &permission))
            {
                Ok(())
            } else {
                Err(::mimic::api::auth::AuthError::NotPermitted {
                    id,
                    permission,
                })
                .map_err(::mimic::api::Error::from)
                .map_err(::mimic::Error::from)
                .map_err(Error::from)
            }
        }
    };

    builder.extend_actor(q);
}
