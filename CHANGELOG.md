# waSCC Host Change Log

All notable changes to this project will be documented in this file.

_The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)_

## [0.12.0] - ?? AUG ??

This version corresponds to the project milestone 0.12

### Added

* _Host Labels_ - You can now add arbitrary key/value pairs to a host manifest (when feature enabled) or via the `set_label` function (when `lattice` is enabled). These labels are discoverable via lattice host probe query. The host runtime will automatically add the appropriate values for the following reserved labels which cannot be overridden:

    * hostcore.os
    * hostcore.osfamily
    * hostcore.arch

* _Uptime_ - Uptime is now being tracked when the `lattice` feature is enabled, and will be reported in response to host probes on the control subject.

### Changed

* The `wascc_host` binary will now default its log level to `INFO`, and you can override this behavior with the standard `RUST_LOG` environment variable syntax.
* The crate's `Error` type now requires `Send` and `Sync`. This should have very little impact on consumers.

## [0.11.0] - 2020 JUL 24

This version lays the groundwork for providing more advanced and pluggable authorization functionality in the future.

### Removed

* The `set_auth_hook` function has been removed from the `WasccHost` struct

### Added

* The `with_authorizer` function has been added to `WasccHost` to allow a developer to create an instance of a waSCC host with a custom authorizer (anything that implements the new `Authorizer` trait). While waSCC host will _always_ enforce the core capability claims when validating whether an actor can communicate with a given capability provider, the new `Authorizer` allows developers to build custom code that further constrains / limits the authorization when loading actors into a host and when actors are attempting to invoke operations against other actors or capability providers.

## [0.10.0] - 2020 JUL 9

This release includes several lattice-related enhancements as well as some security and stability improvements.

### Changed

* The `Invocation` type now includes its own set of claims that must be verified by receiving code. This prevents invocations from being forged on the wire in the case of intrusion.
* The `InvocationTarget` enum has been renamed to `WasccEntity` to better clarify the expected communications patterns
* Middleware now has the ability to indicate a stop or a short-circuit in the middleware change. The trait signature for middleware has changed and any middleware structs built against 0.9.0 will have to be upgraded.

### Added

* Each waSCC host instance now generates its own unique signing key (of type server, nkey prefix is `N` for "node"). This signing key is used to mint forge-proof invocations for transmission over the lattice.
* The host now announces (at `info` level) to the stdout log its version number.
* All waSCC hosts in lattice mode will now perform an antiforgery check on inbound invocations.
* All waSCC hosts in lattice mode will now respond to inventory requests allowing authorized clients to probe the lattice for actors, capabilities, bindings, and hosts.
* The waSCC host will now supply a number of additional actor claims (name, capabilities, tags, expiration, and issuer) to the capability provider during the binding in the form of custom key-value pairs added to the configuration hash map. For the list of these new keys, see [waSCC Codec](../wascc-codec).

## [0.8.0] - 2020 JUN 8

This release was primarily to accomodate the upgrade to the newest version of the [waSCC Codec](../wascc-codec).

### Changed

All capability providers (including _portable_ WASI providers) are now required to respond to the operation `OP_GET_CAPABILITY_DESCRIPTOR` and return a messagepack-serialized struct containing metadata about the capability provider. This metadata includes:

* Name
* Documentation description
* Version (semver string) and Revision (monotonic)
* List of supported operations

We created a simple _builder_ syntax that makes it easy and readable for capability providers to supply a capability descriptor:

```rust
/// Obtains the capability provider descriptor
fn get_descriptor(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serialize(
        CapabilityDescriptor::builder()
            .id(CAPABILITY_ID)
            .name("Default waSCC HTTP Server Provider (Actix)")
            .long_description("A fast, multi-threaded HTTP server for waSCC actors")
            .version(VERSION)
            .revision(REVISION)
            .with_operation(
                OP_HANDLE_REQUEST,
                OperationDirection::ToActor,
                "Delivers an HTTP request to an actor and expects an HTTP response in return",
            )
            .build(),
    )?)
}
```

**NOTE** - This is a breaking change, so old versions of capability providers will _not_ work with this version of the waSCC host.
