// Copyright (c) 2023 Elektrobit Automotive GmbH
//
// This program and the accompanying materials are made available under the
// terms of the Apache License, Version 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.
//
// SPDX-License-Identifier: Apache-2.0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/ankaios.proto")?;
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/control_interface_api.proto", "proto/grpc_api.proto"],
            &["proto"],
        )
        .unwrap();
    Ok(())
}
