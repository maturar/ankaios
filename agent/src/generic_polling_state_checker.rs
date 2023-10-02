use async_trait::async_trait;
use std::time::Duration;
use tokio::{task::JoinHandle, time};

use crate::state_checker::{RuntimeStateChecker, StateChecker};
use common::{
    objects::{ExecutionState, WorkloadInstanceName, WorkloadSpec},
    state_change_interface::{StateChangeInterface, StateChangeSender},
    std_extensions::IllegalStateResult,
};

// [impl->swdd~podman-workload-monitor-interval~1]
const STATUS_CHECK_INTERVAL_MS: u64 = 1000;

#[derive(Debug)]
pub struct GenericPollingStateChecker {
    pub task_handle: Option<JoinHandle<()>>,
}

#[async_trait]
impl StateChecker for GenericPollingStateChecker {
    fn start_checker(
        workload_spec: &WorkloadSpec,
        manager_interface: StateChangeSender,
        state_checker: impl RuntimeStateChecker,
    ) -> Self {
        let workload_spec = workload_spec.clone();
        let task_handle = tokio::spawn(async move {
            let mut last_state = ExecutionState::ExecUnknown;
            let mut interval = time::interval(Duration::from_millis(STATUS_CHECK_INTERVAL_MS));
            let instance_name = workload_spec.instance_name();
            loop {
                interval.tick().await;
                let current_state = state_checker.check_state(&instance_name).await;

                if current_state != last_state {
                    log::info!(
                        "The workload {} has changed its state to {:?}",
                        workload_spec.name,
                        current_state
                    );
                    last_state = current_state.clone();

                    // [impl->swdd~podman-workload-sends-workload-state~1]
                    manager_interface
                        .update_workload_state(vec![common::objects::WorkloadState {
                            agent_name: workload_spec.agent.clone(),
                            workload_name: workload_spec.name.to_string(),
                            execution_state: current_state,
                        }])
                        .await
                        .unwrap_or_illegal_state();

                    if last_state == ExecutionState::ExecRemoved {
                        break;
                    }
                }
            }
        });

        GenericPollingStateChecker {
            task_handle: Some(task_handle),
        }
    }

    async fn stop_checker(self) {
        self.task_handle.map(|x| x.abort());
    }
}
