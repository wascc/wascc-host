// Copyright 2015-2020 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use wascc_host::{Actor, NativeCapability, WasccHost, WasiParams};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let host = WasccHost::new();
    host.add_actor(Actor::from_file("./examples/.assets/wasi_consumer.wasm")?)?;
    host.add_capability(
        Actor::from_file("./examples/.assets/wasi_provider.wasm")?,
        None,
        WasiParams::default(),
    )?; // WASI default does not map file system access

    host.add_native_capability(NativeCapability::from_file(
        "./examples/.assets/libwascc_httpsrv.so",
        None,
    )?)?;

    host.bind_actor(
        "MDNPIQOU5EEHTP4TKY2APFOJTTEYYARN3ZIJTRWRYWHX6B4MFSO6ZCRT",
        "wascc:wasidemo",
        None,
        HashMap::new(),
    )?;

    host.bind_actor(
        "MDNPIQOU5EEHTP4TKY2APFOJTTEYYARN3ZIJTRWRYWHX6B4MFSO6ZCRT",
        "wascc:http_server",
        None,
        generate_port_config(8081),
    )?;

    for (_rk, descriptor) in host.capabilities() {
        println!("  **  Capability providers in Host:\n");
        println!(
            "\t'{}' v{} ({}) for {}",
            descriptor.name, descriptor.version, descriptor.revision, descriptor.id
        );
    }
    for (actor, _claims) in host.actors() {
        println!("  **  Actors in Host:\n");
        println!("\t{}", actor);
    }

    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());

    hm
}
