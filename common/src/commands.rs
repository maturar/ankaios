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

use crate::objects::{DeletedWorkload, State, WorkloadSpec, WorkloadState};
use api::proto;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AgentHello {
    pub agent_name: String,
}

impl From<proto::AgentHello> for AgentHello {
    fn from(item: proto::AgentHello) -> Self {
        AgentHello {
            agent_name: item.agent_name,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AgentGone {
    pub agent_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateWorkloadState {
    pub workload_states: Vec<crate::objects::WorkloadState>,
}

impl From<UpdateWorkloadState> for proto::UpdateWorkloadState {
    fn from(item: UpdateWorkloadState) -> Self {
        proto::UpdateWorkloadState {
            workload_states: item.workload_states.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<proto::UpdateWorkloadState> for UpdateWorkloadState {
    fn from(item: proto::UpdateWorkloadState) -> Self {
        UpdateWorkloadState {
            workload_states: item.workload_states.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub request_id: String,
    pub request_content: RequestContent,
}

impl From<Request> for proto::Request {
    fn from(value: Request) -> Self {
        Self {
            request_id: value.request_id,
            request_content: Some(value.request_content.into()),
        }
    }
}

impl Request {
    pub fn prefix_request_id(&mut self, prefix: &str) {
        self.request_id = format!("{}{}", prefix, self.request_id);
    }
}

impl TryFrom<proto::Request> for Request {
    type Error = String;
    fn try_from(value: proto::Request) -> Result<Request, Self::Error> {
        Ok(Request {
            request_id: value.request_id,
            request_content: value
                .request_content
                .ok_or_else(|| "Request has no content".to_string())?
                .try_into()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestContent {
    CompleteStateRequest(CompleteStateRequest),
    UpdateStateRequest(Box<UpdateStateRequest>),
}

impl From<RequestContent> for proto::request::RequestContent {
    fn from(value: RequestContent) -> Self {
        match value {
            RequestContent::CompleteStateRequest(content) => {
                proto::request::RequestContent::CompleteStateRequest(content.into())
            }
            RequestContent::UpdateStateRequest(content) => {
                proto::request::RequestContent::UpdateStateRequest((*content).into())
            }
        }
    }
}

impl TryFrom<proto::request::RequestContent> for RequestContent {
    type Error = String;
    fn try_from(value: proto::request::RequestContent) -> Result<Self, Self::Error> {
        Ok(match value {
            proto::request::RequestContent::UpdateStateRequest(value) => {
                RequestContent::UpdateStateRequest(Box::new(value.try_into()?))
            }
            proto::request::RequestContent::CompleteStateRequest(value) => {
                RequestContent::CompleteStateRequest(value.into())
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteStateRequest {
    pub field_mask: Vec<String>,
}

impl From<CompleteStateRequest> for proto::CompleteStateRequest {
    fn from(item: CompleteStateRequest) -> Self {
        proto::CompleteStateRequest {
            field_mask: item.field_mask,
        }
    }
}

impl From<proto::CompleteStateRequest> for CompleteStateRequest {
    fn from(item: proto::CompleteStateRequest) -> Self {
        CompleteStateRequest {
            field_mask: item.field_mask,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UpdateStateRequest {
    pub state: CompleteState,
    pub update_mask: Vec<String>,
}

impl From<UpdateStateRequest> for proto::UpdateStateRequest {
    fn from(value: UpdateStateRequest) -> Self {
        Self {
            new_state: Some(value.state.into()),
            update_mask: value.update_mask,
        }
    }
}

impl TryFrom<proto::UpdateStateRequest> for UpdateStateRequest {
    type Error = String;

    fn try_from(item: proto::UpdateStateRequest) -> Result<Self, Self::Error> {
        Ok(UpdateStateRequest {
            state: item.new_state.unwrap_or_default().try_into()?,
            update_mask: item.update_mask,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateWorkload {
    pub added_workloads: Vec<WorkloadSpec>,
    pub deleted_workloads: Vec<DeletedWorkload>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub response_content: ResponseContent,
}

impl From<Response> for proto::Response {
    fn from(value: Response) -> Self {
        Self {
            request_id: value.request_id,
            response_content: Some(value.response_content.into()),
        }
    }
}

impl TryFrom<proto::Response> for Response {
    type Error = String;

    fn try_from(value: proto::Response) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: value.request_id,
            response_content: value
                .response_content
                .ok_or_else(|| "Response has no content".to_string())?
                .try_into()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ResponseContent {
    Success,
    Error(Error),
    CompleteState(Box<CompleteState>),
    UpdateStateSuccess(UpdateStateSuccess),
}

impl From<ResponseContent> for proto::response::ResponseContent {
    fn from(value: ResponseContent) -> Self {
        match value {
            ResponseContent::Success => {
                proto::response::ResponseContent::Success(proto::Success {})
            }

            ResponseContent::Error(error) => proto::response::ResponseContent::Error(error.into()),
            ResponseContent::CompleteState(complete_state) => {
                proto::response::ResponseContent::CompleteState((*complete_state).into())
            }
            ResponseContent::UpdateStateSuccess(update_state_success) => {
                proto::response::ResponseContent::UpdateStateSuccess(update_state_success.into())
            }
        }
    }
}

impl TryFrom<proto::response::ResponseContent> for ResponseContent {
    type Error = String;

    fn try_from(value: proto::response::ResponseContent) -> Result<Self, String> {
        match value {
            proto::response::ResponseContent::Success(_) => Ok(ResponseContent::Success),
            proto::response::ResponseContent::Error(error) => {
                Ok(ResponseContent::Error(error.into()))
            }
            proto::response::ResponseContent::CompleteState(complete_state) => Ok(
                ResponseContent::CompleteState(Box::new(complete_state.try_into()?)),
            ),
            proto::response::ResponseContent::UpdateStateSuccess(update_state_success) => Ok(
                ResponseContent::UpdateStateSuccess(update_state_success.into()),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct Error {
    pub message: String,
}

impl From<proto::Error> for Error {
    fn from(value: proto::Error) -> Self {
        Self {
            message: value.message,
        }
    }
}

impl From<Error> for proto::Error {
    fn from(value: Error) -> Self {
        proto::Error {
            message: value.message,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct CompleteState {
    pub startup_state: State,
    pub current_state: State,
    pub workload_states: Vec<WorkloadState>,
}

impl From<CompleteState> for proto::CompleteState {
    fn from(item: CompleteState) -> proto::CompleteState {
        proto::CompleteState {
            startup_state: Some(proto::State::from(item.startup_state)),
            current_state: Some(proto::State::from(item.current_state)),
            workload_states: item.workload_states.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl TryFrom<proto::CompleteState> for CompleteState {
    type Error = String;

    fn try_from(item: proto::CompleteState) -> Result<Self, Self::Error> {
        Ok(CompleteState {
            startup_state: item.startup_state.unwrap_or_default().try_into()?,
            current_state: item.current_state.unwrap_or_default().try_into()?,
            workload_states: item.workload_states.into_iter().map(|x| x.into()).collect(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct UpdateStateSuccess {
    pub added_workloads: Vec<String>,
    pub deleted_workloads: Vec<String>,
}

impl From<UpdateStateSuccess> for proto::UpdateStateSuccess {
    fn from(value: UpdateStateSuccess) -> Self {
        Self {
            added_workloads: value.added_workloads,
            deleted_workloads: value.deleted_workloads,
        }
    }
}

impl From<proto::UpdateStateSuccess> for UpdateStateSuccess {
    fn from(value: proto::UpdateStateSuccess) -> Self {
        Self {
            added_workloads: value.added_workloads,
            deleted_workloads: value.deleted_workloads,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Goodbye {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Stop {}

//////////////////////////////////////////////////////////////////////////////
//                 ########  #######    #########  #########                //
//                    ##     ##        ##             ##                    //
//                    ##     #####     #########      ##                    //
//                    ##     ##                ##     ##                    //
//                    ##     #######   #########      ##                    //
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    mod proto {
        pub use api::proto::{
            request::RequestContent, response::ResponseContent, CompleteState,
            CompleteStateRequest, Error, ExecutionState, Request, Response, State, Success,
            UpdateStateRequest, UpdateStateSuccess, UpdateWorkloadState, Workload, WorkloadState,
        };
    }

    mod ankaios {
        pub use crate::{
            commands::{
                CompleteState, CompleteStateRequest, Error, Request, RequestContent, Response,
                ResponseContent, UpdateStateRequest, UpdateStateSuccess, UpdateWorkloadState,
            },
            objects::{ExecutionState, State, WorkloadState},
        };
    }

    const REQUEST_ID: &str = "request_id";
    const FIELD_1: &str = "field_1";
    const FIELD_2: &str = "field_2";
    const AGENT_NAME: &str = "agent_1";
    const WORKLOAD_NAME_1: &str = "workload_name_1";
    const WORKLOAD_NAME_2: &str = "workload_name_2";
    const WORKLOAD_NAME_3: &str = "workload_name_3";
    const ERROR_MESSAGE: &str = "error_message";

    macro_rules! update_workload_state {
        ($expression:path) => {{
            use $expression::{ExecutionState, UpdateWorkloadState, WorkloadState};

            UpdateWorkloadState {
                workload_states: vec![WorkloadState {
                    workload_name: "john".to_string(),
                    agent_name: "doe".to_string(),
                    execution_state: ExecutionState::ExecRunning.into(),
                }],
            }
        }};
    }

    macro_rules! complete_state_request {
        ($expression:path) => {{
            use $expression::{CompleteStateRequest, Request, RequestContent};

            Request {
                request_id: REQUEST_ID.into(),
                request_content: RequestContent::CompleteStateRequest(CompleteStateRequest {
                    field_mask: vec![FIELD_1.into(), FIELD_2.into()],
                })
                .into(),
            }
        }};
    }

    #[rustfmt::skip]
    macro_rules! update_state_request {
        ($expression:ident) => {{
            $expression::Request {
                request_id: REQUEST_ID.into(),
                request_content: update_state_request_enum!($expression).into(),
            }
        }};
    }

    macro_rules! update_state_request_enum {
        (proto) => {
            proto::RequestContent::UpdateStateRequest(proto::UpdateStateRequest {
                new_state: complete_state!(proto).into(),
                update_mask: vec![FIELD_1.into(), FIELD_2.into()],
            })
        };
        (ankaios) => {
            ankaios::RequestContent::UpdateStateRequest(Box::new(ankaios::UpdateStateRequest {
                state: complete_state!(ankaios),
                update_mask: vec![FIELD_1.into(), FIELD_2.into()],
            }))
        };
    }

    macro_rules! success_response {
        (proto) => {
            proto::Response {
                request_id: REQUEST_ID.into(),
                response_content: proto::ResponseContent::Success(proto::Success {}).into(),
            }
        };
        (ankaios) => {
            ankaios::Response {
                request_id: REQUEST_ID.into(),
                response_content: ankaios::ResponseContent::Success,
            }
        };
    }

    macro_rules! error_response {
        ($expression:path) => {{
            use $expression::{Error, Response, ResponseContent};
            Response {
                request_id: REQUEST_ID.into(),
                response_content: ResponseContent::Error(Error {
                    message: ERROR_MESSAGE.into(),
                })
                .into(),
            }
        }};
    }

    macro_rules! complete_state_response {
        ($expression:path) => {{
            use $expression::{Response, ResponseContent};
            Response {
                request_id: REQUEST_ID.into(),
                response_content: ResponseContent::CompleteState(
                    complete_state!($expression).into(),
                )
                .into(),
            }
        }};
    }

    macro_rules! complete_state {
        // Use extra block as scope for the `use`
        ($expression:path) => {{
            use $expression::{CompleteState, ExecutionState, State, WorkloadState};
            CompleteState {
                startup_state: State {
                    configs: [("startup".into(), "state".into())].into(),
                    ..Default::default()
                }
                .into(),
                current_state: State {
                    configs: [("current".into(), "state".into())].into(),
                    ..Default::default()
                }
                .into(),
                workload_states: vec![WorkloadState {
                    workload_name: WORKLOAD_NAME_1.into(),
                    agent_name: AGENT_NAME.into(),
                    execution_state: ExecutionState::ExecRunning.into(),
                }],
            }
        }};
    }

    macro_rules! update_state_success_response {
        ($expression:path) => {{
            use $expression::{Response, ResponseContent, UpdateStateSuccess};

            Response {
                request_id: REQUEST_ID.into(),
                response_content: ResponseContent::UpdateStateSuccess(UpdateStateSuccess {
                    added_workloads: vec![WORKLOAD_NAME_1.into()],
                    deleted_workloads: vec![WORKLOAD_NAME_2.into(), WORKLOAD_NAME_3.into()],
                })
                .into(),
            }
        }};
    }

    #[test]
    fn utest_converts_to_proto_update_workload_state() {
        let ankaios_update_wl_state = update_workload_state!(ankaios);
        let proto_update_wl_state = update_workload_state!(proto);

        assert_eq!(
            proto::UpdateWorkloadState::from(ankaios_update_wl_state),
            proto_update_wl_state
        );
    }

    #[test]
    fn utest_converts_from_proto_update_workload_state() {
        let proto_update_wl_state = update_workload_state!(proto);
        let ankaios_update_wl_state = update_workload_state!(ankaios);

        assert_eq!(
            ankaios::UpdateWorkloadState::from(proto_update_wl_state),
            ankaios_update_wl_state,
        );
    }

    #[test]
    fn utest_converts_to_proto_complete_state_request() {
        let ankaios_request_complete_state = complete_state_request!(ankaios);
        let proto_request_complete_state = complete_state_request!(proto);

        assert_eq!(
            proto::Request::from(ankaios_request_complete_state),
            proto_request_complete_state
        );
    }

    #[test]
    fn utest_converts_from_proto_complete_state_request() {
        let proto_request_complete_state = complete_state_request!(proto);
        let ankaios_request_complete_state = complete_state_request!(ankaios);

        assert_eq!(
            ankaios::Request::try_from(proto_request_complete_state).unwrap(),
            ankaios_request_complete_state
        );
    }

    #[test]
    fn utest_converts_to_proto_update_state_request() {
        let ankaios_request_complete_state = update_state_request!(ankaios);
        let proto_request_complete_state = update_state_request!(proto);

        assert_eq!(
            proto::Request::from(ankaios_request_complete_state),
            proto_request_complete_state
        );
    }

    #[test]
    fn utest_converts_from_proto_update_state_request() {
        let proto_request_complete_state = update_state_request!(proto);
        let ankaios_request_complete_state = update_state_request!(ankaios);

        assert_eq!(
            ankaios::Request::try_from(proto_request_complete_state).unwrap(),
            ankaios_request_complete_state
        );
    }

    #[test]
    fn utest_converts_from_proto_update_state_request_with_empty_states() {
        let mut proto_request_complete_state = update_state_request!(proto);
        let mut ankaios_request_complete_state = update_state_request!(ankaios);

        let proto::RequestContent::UpdateStateRequest(proto_request_content) =
            proto_request_complete_state
                .request_content
                .as_mut()
                .unwrap()
        else {
            unreachable!()
        };
        proto_request_content.new_state = None;

        let ankaios::RequestContent::UpdateStateRequest(ankaios_request_content) =
            &mut ankaios_request_complete_state.request_content
        else {
            unreachable!()
        };
        ankaios_request_content.state = Default::default();

        assert_eq!(
            ankaios::Request::try_from(proto_request_complete_state).unwrap(),
            ankaios_request_complete_state
        );
    }

    #[test]
    fn utest_converts_from_proto_update_state_request_inner_state_with_empty_states() {
        let mut proto_request_complete_state = update_state_request!(proto);
        let mut ankaios_request_complete_state = update_state_request!(ankaios);

        let proto::RequestContent::UpdateStateRequest(proto_request_content) =
            proto_request_complete_state
                .request_content
                .as_mut()
                .unwrap()
        else {
            unreachable!()
        };
        proto_request_content
            .new_state
            .as_mut()
            .unwrap()
            .startup_state = None;
        proto_request_content
            .new_state
            .as_mut()
            .unwrap()
            .current_state = None;

        let ankaios::RequestContent::UpdateStateRequest(ankaios_request_content) =
            &mut ankaios_request_complete_state.request_content
        else {
            unreachable!()
        };
        ankaios_request_content.state.startup_state = Default::default();
        ankaios_request_content.state.current_state = Default::default();

        assert_eq!(
            ankaios::Request::try_from(proto_request_complete_state).unwrap(),
            ankaios_request_complete_state
        );
    }

    #[test]
    fn utest_converts_from_proto_update_state_request_fails_invalid_startup_state() {
        let mut proto_request_complete_state = update_state_request!(proto);

        let proto::RequestContent::UpdateStateRequest(proto_request_content) =
            proto_request_complete_state
                .request_content
                .as_mut()
                .unwrap()
        else {
            unreachable!()
        };
        proto_request_content
            .new_state
            .as_mut()
            .unwrap()
            .startup_state
            .as_mut()
            .unwrap()
            .workloads
            .insert(
                WORKLOAD_NAME_1.into(),
                proto::Workload {
                    update_strategy: -1,
                    ..Default::default()
                },
            );

        assert!(ankaios::Request::try_from(proto_request_complete_state).is_err());
    }

    #[test]
    fn utest_converts_from_proto_update_state_request_fails_invalid_current_state() {
        let mut proto_request_complete_state = update_state_request!(proto);

        let proto::RequestContent::UpdateStateRequest(proto_request_content) =
            proto_request_complete_state
                .request_content
                .as_mut()
                .unwrap()
        else {
            unreachable!()
        };
        proto_request_content
            .new_state
            .as_mut()
            .unwrap()
            .current_state
            .as_mut()
            .unwrap()
            .workloads
            .insert(
                WORKLOAD_NAME_1.into(),
                proto::Workload {
                    update_strategy: -1,
                    ..Default::default()
                },
            );

        assert!(ankaios::Request::try_from(proto_request_complete_state).is_err());
    }

    #[test]
    fn utest_converts_from_proto_request_fails_empty_request_content() {
        let proto_request = proto::Request {
            request_id: REQUEST_ID.into(),
            request_content: None,
        };

        assert_eq!(
            ankaios::Request::try_from(proto_request).unwrap_err(),
            "Request has no content"
        );
    }

    #[test]
    fn utest_converts_to_proto_success_response() {
        let ankaios_success_response = success_response!(ankaios);
        let proto_success_response = success_response!(proto);

        assert_eq!(
            proto::Response::from(ankaios_success_response),
            proto_success_response
        );
    }

    #[test]
    fn utest_converts_from_proto_success_response() {
        let proto_success_response = success_response!(proto);
        let ankaios_success_response = success_response!(ankaios);

        assert_eq!(
            ankaios::Response::try_from(proto_success_response).unwrap(),
            ankaios_success_response
        );
    }

    #[test]
    fn utest_converts_to_proto_error_response() {
        let ankaios_error_response = error_response!(ankaios);
        let proto_error_response = error_response!(proto);

        assert_eq!(
            proto::Response::from(ankaios_error_response),
            proto_error_response
        );
    }

    #[test]
    fn utest_converts_from_proto_error_response() {
        let proto_error_response = error_response!(proto);
        let ankaios_error_response = error_response!(ankaios);

        assert_eq!(
            ankaios::Response::try_from(proto_error_response).unwrap(),
            ankaios_error_response,
        );
    }

    #[test]
    fn utest_converts_to_proto_complete_state_response() {
        let ankaios_complete_state_response = complete_state_response!(ankaios);
        let proto_complete_state_response = complete_state_response!(proto);

        assert_eq!(
            proto::Response::from(ankaios_complete_state_response),
            proto_complete_state_response
        );
    }

    #[test]
    fn utest_converts_from_proto_complete_state_response() {
        let proto_complete_state_response = complete_state_response!(proto);
        let ankaios_complete_state_response = complete_state_response!(ankaios);

        assert_eq!(
            ankaios::Response::try_from(proto_complete_state_response).unwrap(),
            ankaios_complete_state_response
        );
    }

    #[test]
    fn utest_converts_from_proto_complete_state_response_with_empty_states() {
        let mut proto_complete_state_response = complete_state_response!(proto);
        let mut ankaios_complete_state_response = complete_state_response!(ankaios);

        let proto::ResponseContent::CompleteState(proto_content) = proto_complete_state_response
            .response_content
            .as_mut()
            .unwrap()
        else {
            unreachable!()
        };
        proto_content.startup_state = None;
        proto_content.current_state = None;

        let ankaios::ResponseContent::CompleteState(ankaios_content) =
            &mut ankaios_complete_state_response.response_content
        else {
            unreachable!()
        };
        ankaios_content.startup_state = Default::default();
        ankaios_content.current_state = Default::default();

        assert_eq!(
            ankaios::Response::try_from(proto_complete_state_response).unwrap(),
            ankaios_complete_state_response
        );
    }

    #[test]
    fn utest_converts_from_proto_complete_state_response_fails_invalid_startup_state() {
        let mut proto_complete_state_response = complete_state_response!(proto);

        let proto::ResponseContent::CompleteState(proto_request_content) =
            proto_complete_state_response
                .response_content
                .as_mut()
                .unwrap()
        else {
            unreachable!()
        };

        proto_request_content
            .startup_state
            .as_mut()
            .unwrap()
            .workloads
            .insert(
                WORKLOAD_NAME_1.into(),
                proto::Workload {
                    update_strategy: -1,
                    ..Default::default()
                },
            );

        assert!(ankaios::Response::try_from(proto_complete_state_response).is_err());
    }

    #[test]
    fn utest_converts_to_proto_update_state_success_response() {
        let ankaios_complete_state_response = update_state_success_response!(ankaios);
        let proto_complete_state_response = update_state_success_response!(proto);

        assert_eq!(
            proto::Response::from(ankaios_complete_state_response),
            proto_complete_state_response
        );
    }

    #[test]
    fn utest_converts_from_proto_update_state_success_response() {
        let proto_complete_state_response = update_state_success_response!(proto);
        let ankaios_complete_state_response = update_state_success_response!(ankaios);

        assert_eq!(
            ankaios::Response::try_from(proto_complete_state_response).unwrap(),
            ankaios_complete_state_response
        );
    }

    #[test]
    fn utest_converts_from_proto_reponse_fails_empty_request_content() {
        let proto_response = proto::Response {
            request_id: REQUEST_ID.into(),
            response_content: None,
        };

        assert_eq!(
            ankaios::Response::try_from(proto_response).unwrap_err(),
            "Response has no content"
        );
    }
    #[test]
    fn utest_request_complete_state_prefix_request_id() {
        let mut ankaios_request_complete_state = ankaios::Request {
            request_id: "42".to_string(),
            request_content: ankaios::RequestContent::CompleteStateRequest(
                ankaios::CompleteStateRequest {
                    field_mask: vec!["1".to_string(), "2".to_string()],
                },
            ),
        };

        ankaios_request_complete_state.prefix_request_id("prefix@");

        assert_eq!("prefix@42", ankaios_request_complete_state.request_id);
    }
}
