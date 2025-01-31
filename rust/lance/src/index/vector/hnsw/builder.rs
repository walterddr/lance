// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Lance Authors

use std::sync::Arc;

use arrow::{array::AsArray, datatypes::Float32Type};
use arrow_array::Array;
use lance_core::Result;
use lance_index::vector::{
    graph::memory::InMemoryVectorStorage,
    hnsw::{builder::HnswBuildParams, builder::HNSW},
};
use lance_linalg::{distance::DistanceType, MatrixView};

pub async fn build_hnsw_model(
    hnsw_params: HnswBuildParams,
    vectors: Arc<dyn Array>,
) -> Result<HNSW> {
    let mat = MatrixView::<Float32Type>::try_from(vectors.as_fixed_size_list())?;

    // We have normalized the vectors if the metric type is cosine, so we can use the L2 distance
    let vec_store = Arc::new(InMemoryVectorStorage::new(mat, DistanceType::L2));
    HNSW::build_with_storage(DistanceType::L2, hnsw_params, vec_store).await
}
