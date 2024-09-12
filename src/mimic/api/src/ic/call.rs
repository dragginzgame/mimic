use candid::{
    decode_args, encode_args,
    utils::{ArgumentDecoder, ArgumentEncoder},
    CandidType, Principal,
};
use ic::{
    api::call::{call_raw, RejectionCode},
    log, Log,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::future::Future;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("candid error: {error}"))]
    Candid { error: String },

    #[snafu(display("call rejected: {error}"))]
    CallRejected { error: String },
}

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self::CallRejected { error: error.1 }
    }
}

//
// call
// wrapping this because otherwise the error is a pain to handle
//

pub fn call<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    id: Principal,
    method: &str,
    args: T,
) -> impl Future<Output = Result<R, Error>> + Send + Sync {
    log!(Log::Info, "call: {method}@{id}");

    let args_raw = encode_args(args).expect("Failed to encode arguments.");
    let fut = call_raw(id, method, args_raw, 0);

    async {
        let bytes: Vec<u8> = fut.await.map_err(Error::from)?;

        let res = decode_args(&bytes).map_err(|e| Error::Candid {
            error: e.to_string(),
        })?;

        Ok(res)
    }
}
