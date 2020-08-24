use crate::Result;
use crate::{plugins::PluginManager, Invocation, InvocationResponse};
use std::sync::Arc;
use std::sync::RwLock;
use wapc::WapcHost;

#[cfg(feature = "prometheus_middleware")]
pub mod prometheus;

/// The trait that must be implemented by all waSCC middleware
pub trait Middleware: Send + Sync + 'static {
    fn actor_pre_invoke(&self, inv: Invocation) -> Result<Invocation>;
    fn actor_invoke(
        &self,
        inv: Invocation,
        handler: InvocationHandler,
    ) -> Result<MiddlewareResponse>;
    fn actor_post_invoke(&self, response: InvocationResponse) -> Result<InvocationResponse>;

    fn capability_pre_invoke(&self, inv: Invocation) -> Result<Invocation>;
    fn capability_invoke(
        &self,
        inv: Invocation,
        handler: InvocationHandler,
    ) -> Result<MiddlewareResponse>;
    fn capability_post_invoke(&self, response: InvocationResponse) -> Result<InvocationResponse>;
}

pub enum MiddlewareResponse {
    Continue(InvocationResponse),
    Halt(InvocationResponse),
}

pub struct InvocationHandler<'a> {
    operation: &'a dyn Fn(Invocation) -> InvocationResponse,
}

impl<'a> InvocationHandler<'a> {
    fn new(operation: &'a dyn Fn(Invocation) -> InvocationResponse) -> Self {
        Self { operation }
    }

    pub fn invoke(&self, inv: Invocation) -> InvocationResponse {
        (self.operation)(inv)
    }
}

/// Follows a chain of middleware, ultimately executing the native plugin
pub(crate) fn invoke_native_capability(
    middlewares: Arc<RwLock<Vec<Box<dyn Middleware>>>>,
    inv: Invocation,
    plugins: Arc<RwLock<PluginManager>>,
) -> Result<InvocationResponse> {
    let inv = match run_capability_pre_invoke(inv.clone(), &middlewares.read().unwrap()) {
        Ok(i) => i,
        Err(e) => {
            error!("Middleware failure: {}", e);
            inv
        }
    };

    match run_native_capability_invoke(&middlewares.read().unwrap(), &plugins.read().unwrap(), inv)
    {
        Ok(response) => {
            match run_capability_post_invoke(response.clone(), &middlewares.read().unwrap()) {
                Ok(r) => Ok(r),
                Err(e) => {
                    error!("Middleware failure: {}", e);
                    Ok(response)
                }
            }
        }
        Err(e) => Err(e),
    }
}

/// Follows a chain of middleware, ultimately executing a portable capability provider function
pub(crate) fn invoke_portable_capability(
    middlewares: Arc<RwLock<Vec<Box<dyn Middleware>>>>,
    inv: Invocation,
    guest: &WapcHost,
) -> Result<InvocationResponse> {
    let inv = match run_capability_pre_invoke(inv.clone(), &middlewares.read().unwrap()) {
        Ok(i) => i,
        Err(e) => {
            error!("Middleware failure: {}", e);
            inv
        }
    };

    match run_portable_capability_invoke(&middlewares.read().unwrap(), inv, guest) {
        Ok(response) => {
            match run_capability_post_invoke(response.clone(), &middlewares.read().unwrap()) {
                Ok(r) => Ok(r),
                Err(e) => {
                    error!("Middleware failure: {}", e);
                    Ok(response)
                }
            }
        }
        Err(e) => Err(e),
    }
}

pub(crate) fn invoke_actor(
    middlewares: Arc<RwLock<Vec<Box<dyn Middleware>>>>,
    inv: Invocation,
    guest: &WapcHost,
) -> Result<InvocationResponse> {
    let inv = match run_actor_pre_invoke(inv.clone(), &middlewares.read().unwrap()) {
        Ok(i) => i,
        Err(e) => {
            error!("Middleware failure: {}", e);
            inv
        }
    };

    match run_actor_invoke(&middlewares.read().unwrap(), inv, guest) {
        Ok(response) => {
            match run_actor_post_invoke(response.clone(), &middlewares.read().unwrap()) {
                Ok(r) => Ok(r),
                Err(e) => {
                    error!("Middleware failure: {}", e);
                    Ok(response)
                }
            }
        }
        Err(e) => Err(e),
    }
}

fn run_actor_pre_invoke(
    inv: Invocation,
    middlewares: &[Box<dyn Middleware>],
) -> Result<Invocation> {
    let mut cur_inv = inv;
    for m in middlewares {
        match m.actor_pre_invoke(cur_inv) {
            Ok(i) => cur_inv = i.clone(),
            Err(e) => return Err(e),
        }
    }
    Ok(cur_inv)
}

fn run_actor_invoke(
    middlewares: &[Box<dyn Middleware>],
    inv: Invocation,
    guest: &WapcHost,
) -> Result<InvocationResponse> {
    let invoke_operation = |inv: Invocation| match guest.call(&inv.operation, &inv.msg) {
        Ok(v) => InvocationResponse::success(&inv, v),
        Err(e) => InvocationResponse::error(&inv, &format!("failed to invoke actor: {}", e)),
    };

    run_invoke(middlewares, inv, &invoke_operation)
}

fn run_actor_post_invoke(
    resp: InvocationResponse,
    middlewares: &[Box<dyn Middleware>],
) -> Result<InvocationResponse> {
    let mut cur_resp = resp;
    for m in middlewares {
        match m.actor_post_invoke(cur_resp) {
            Ok(i) => cur_resp = i.clone(),
            Err(e) => return Err(e),
        }
    }
    Ok(cur_resp)
}

pub(crate) fn run_capability_pre_invoke(
    inv: Invocation,
    middlewares: &[Box<dyn Middleware>],
) -> Result<Invocation> {
    let mut cur_inv = inv;
    for m in middlewares {
        match m.capability_pre_invoke(cur_inv) {
            Ok(i) => cur_inv = i.clone(),
            Err(e) => return Err(e),
        }
    }
    Ok(cur_inv)
}

pub(crate) fn run_native_capability_invoke(
    middlewares: &[Box<dyn Middleware>],
    plugins: &PluginManager,
    inv: Invocation,
) -> Result<InvocationResponse> {
    let invoke_operation = |inv: Invocation| match plugins.call(&inv) {
        Ok(r) => r,
        Err(e) => InvocationResponse::error(&inv, &format!("failed to invoke capability: {}", e)),
    };

    run_invoke(middlewares, inv, &invoke_operation)
}

pub(crate) fn run_portable_capability_invoke(
    middlewares: &[Box<dyn Middleware>],
    inv: Invocation,
    guest: &WapcHost,
) -> Result<InvocationResponse> {
    let invoke_operation = |inv: Invocation| match guest.call(&inv.operation, &inv.msg) {
        Ok(v) => InvocationResponse::success(&inv, v),
        Err(e) => InvocationResponse::error(&inv, &format!("failed to invoke capability: {}", e)),
    };

    run_invoke(middlewares, inv, &invoke_operation)
}

fn run_invoke(
    middlewares: &[Box<dyn Middleware>],
    inv: Invocation,
    invoke_operation: &dyn Fn(Invocation) -> InvocationResponse,
) -> Result<InvocationResponse> {
    let mut cur_resp = Ok(InvocationResponse::error(
        &inv,
        "No middleware invoked the operation",
    ));

    for m in middlewares.iter() {
        match m.capability_invoke(inv.clone(), InvocationHandler::new(&invoke_operation)) {
            Ok(mr) => match mr {
                MiddlewareResponse::Continue(res) => cur_resp = Ok(res),
                MiddlewareResponse::Halt(res) => return Ok(res),
            },
            Err(e) => return Err(e),
        }
    }

    if middlewares.is_empty() {
        Ok(invoke_operation(inv))
    } else {
        cur_resp
    }
}

pub(crate) fn run_capability_post_invoke(
    resp: InvocationResponse,
    middlewares: &[Box<dyn Middleware>],
) -> Result<InvocationResponse> {
    let mut cur_resp = resp;
    for m in middlewares {
        match m.capability_post_invoke(cur_resp) {
            Ok(i) => cur_resp = i.clone(),
            Err(e) => return Err(e),
        }
    }
    Ok(cur_resp)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::Middleware;
    use crate::inthost::{Invocation, InvocationResponse, WasccEntity};
    use crate::middleware::{InvocationHandler, MiddlewareResponse};
    use crate::Result;
    use wascap::prelude::KeyPair;

    struct IncMiddleware {
        pre: &'static AtomicUsize,
        post: &'static AtomicUsize,
        cap_pre: &'static AtomicUsize,
        cap_post: &'static AtomicUsize,
    }

    impl Middleware for IncMiddleware {
        fn actor_pre_invoke(&self, inv: Invocation) -> Result<Invocation> {
            self.pre.fetch_add(1, Ordering::SeqCst);
            Ok(inv)
        }
        fn actor_invoke(
            &self,
            inv: Invocation,
            handler: InvocationHandler,
        ) -> Result<MiddlewareResponse> {
            Ok(MiddlewareResponse::Continue(handler.invoke(inv)))
        }
        fn actor_post_invoke(&self, response: InvocationResponse) -> Result<InvocationResponse> {
            self.post.fetch_add(1, Ordering::SeqCst);
            Ok(response)
        }
        fn capability_pre_invoke(&self, inv: Invocation) -> Result<Invocation> {
            self.cap_pre.fetch_add(1, Ordering::SeqCst);
            Ok(inv)
        }
        fn capability_invoke(
            &self,
            inv: Invocation,
            handler: InvocationHandler,
        ) -> Result<MiddlewareResponse> {
            Ok(MiddlewareResponse::Continue(handler.invoke(inv)))
        }
        fn capability_post_invoke(
            &self,
            response: InvocationResponse,
        ) -> Result<InvocationResponse> {
            self.cap_post.fetch_add(1, Ordering::SeqCst);
            Ok(response)
        }
    }

    static PRE: AtomicUsize = AtomicUsize::new(0);
    static POST: AtomicUsize = AtomicUsize::new(0);
    static CAP_PRE: AtomicUsize = AtomicUsize::new(0);
    static CAP_POST: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn simple_add() {
        let inc_mid = IncMiddleware {
            pre: &PRE,
            post: &POST,
            cap_pre: &CAP_PRE,
            cap_post: &CAP_POST,
        };
        let hk = KeyPair::new_server();

        let mids: Vec<Box<dyn Middleware>> = vec![Box::new(inc_mid)];
        let inv = Invocation::new(
            &hk,
            WasccEntity::Actor("test".to_string()),
            WasccEntity::Capability {
                capid: "testing:sample".to_string(),
                binding: "default".to_string(),
            },
            "testing",
            b"abc1234".to_vec(),
        );
        let res = super::run_actor_pre_invoke(inv.clone(), &mids);
        assert!(res.is_ok());
        let res2 = super::run_actor_pre_invoke(inv, &mids);
        assert!(res2.is_ok());
        assert_eq!(PRE.fetch_add(0, Ordering::SeqCst), 2);
    }
}
