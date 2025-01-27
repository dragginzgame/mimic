pub mod auth;
pub mod core;
pub mod guard;
pub mod ic;
pub mod subnet;

use crate::Error;
use std::future::Future;

///
/// StartupHooks
///

pub trait StartupHooks {
    // startup
    // on every startup regardless of installation mode
    fn startup() -> Result<(), Error> {
        Ok(())
    }

    // init
    // custom code called after mimic init()
    fn init() -> Result<(), Error> {
        Ok(())
    }

    // init_async
    // custom code called after mimic init_async()
    #[must_use]
    fn init_async() -> impl Future<Output = Result<(), Error>> + Send {
        async { Ok(()) }
    }

    // pre_upgrade
    // called after pre_upgrade
    fn pre_upgrade() -> Result<(), Error> {
        Ok(())
    }

    // post_upgrade
    // called after post_upgrade
    fn post_upgrade() -> Result<(), Error> {
        Ok(())
    }
}
