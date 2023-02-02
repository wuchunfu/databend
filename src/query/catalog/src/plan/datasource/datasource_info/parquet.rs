// Copyright 2023 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_arrow::arrow::datatypes::Schema as ArrowSchema;
use common_expression::TableSchema;
use common_meta_app::schema::TableInfo;
use common_meta_types::UserStageInfo;

use crate::plan::datasource::datasource_info::parquet_read_options::ParquetReadOptions;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ParquetTableInfo {
    pub table_info: TableInfo,
    pub file_locations: Vec<String>,
    pub arrow_schema: ArrowSchema,
    pub read_options: ParquetReadOptions,
    pub user_stage_info: UserStageInfo,
}

impl ParquetTableInfo {
    pub fn schema(&self) -> Arc<TableSchema> {
        self.table_info.schema()
    }

    pub fn desc(&self) -> String {
        self.user_stage_info.stage_name.clone()
    }
}
