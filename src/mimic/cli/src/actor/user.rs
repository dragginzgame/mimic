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
            guard(vec![Guard::Controller]).await?;

            Ok(api::state::user_index())
        }

        // get_caller
        // no auth needed as it's just looking up the current caller
        #[::mimic::ic::query]
        fn get_caller() -> Result<User, Error> {
            let user = UserIndexManager::try_get_user(caller())?;

            Ok(user)
        }

        // get_user
        // look up any user by principal, requires an auth check
        #[::mimic::ic::query]
        async fn get_user(id: Principal) -> Result<User, Error> {
            if id != caller() {
                guard(vec![Guard::Controller]).await?;
            }

            let user = UserIndexManager::try_get_user(id)?;

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
            .await?;

            let user = register(id).await?;

            Ok(user)
        }

        // add_role
        #[::mimic::ic::update]
        async fn add_role(id: Principal, role: String) -> Result<(), Error> {
            guard(vec![
                Guard::Parent,
                Guard::Controller,
            ])
            .await?;

            UserIndexManager::add_role(id, role)?;

            Ok(())
        }

        // remove_role
        #[::mimic::ic::update]
        async fn remove_role(id: Principal, role: String) -> Result<(), Error> {
            guard(vec![
                Guard::Parent,
                Guard::Controller,
            ])
            .await?;

            UserIndexManager::remove_role(id, role)?;

            Ok(())
        }

        // guard_permission
        // endpoint only works on the User canister
        #[::mimic::ic::query]
        pub async fn guard_permission(id: Principal, permission: String) -> Result<(), Error> {
            let user = UserIndexManager::try_get_user(id)?;

            // return Ok if any role has the permission, otherwise return an error
            if user
                .roles
                .iter()
                .any(|role| core_schema::AuthService::role_has_permission(role, &permission))
            {
                Ok(())
            } else {
                Err(::mimic::api::auth::AuthError::NotPermitted {
                    id,
                    permission: permission,
                })?
            }
        }
    };

    builder.extend_actor(q);
}
