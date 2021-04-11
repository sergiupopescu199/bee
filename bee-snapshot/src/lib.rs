// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod download;

pub mod config;
pub mod error;
pub mod header;
pub mod info;
pub mod kind;
pub mod milestone_diff;
pub mod reader;
pub mod storage;
pub mod worker;

use bee_runtime::node::{Node, NodeBuilder};

pub async fn init<N: Node>(config: &config::SnapshotConfig, network_id: u64, node_builder: N::Builder) -> N::Builder
where
    N::Backend: storage::StorageBackend,
{
    node_builder.with_worker_cfg::<worker::SnapshotWorker>((network_id, config.clone()))
}
