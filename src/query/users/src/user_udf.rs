// Copyright 2021 Datafuse Labs
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

use databend_common_exception::Result;
use databend_common_management::udf::UdfApiError;
use databend_common_management::udf::UdfError;
use databend_common_meta_app::principal::UserDefinedFunction;
use databend_common_meta_app::schema::CreateOption;
use databend_common_meta_types::MatchSeq;

use crate::UserApiProvider;

/// UDF operations.
impl UserApiProvider {
    // Add a new UDF.
    #[async_backtrace::framed]
    pub async fn add_udf(
        &self,
        tenant: &str,
        info: UserDefinedFunction,
        create_option: &CreateOption,
    ) -> Result<()> {
        let udf_api = self.for_tenant(tenant)?.udf_api();
        udf_api.add_udf(info, create_option).await??;
        Ok(())
    }

    // Update a UDF.
    #[async_backtrace::framed]
    pub async fn update_udf(&self, tenant: &str, info: UserDefinedFunction) -> Result<u64> {
        let res = self
            .for_tenant(tenant)?
            .udf_api()
            .update_udf(info, MatchSeq::GE(1))
            .await?;

        let seq = res?;
        Ok(seq)
    }

    // Get a UDF by name.
    #[async_backtrace::framed]
    pub async fn get_udf(
        &self,
        tenant: &str,
        udf_name: &str,
    ) -> Result<Option<UserDefinedFunction>, UdfApiError> {
        let seqv = self.for_tenant(tenant)?.udf_api().get_udf(udf_name).await?;
        Ok(seqv.map(|x| x.data))
    }

    #[async_backtrace::framed]
    pub async fn exists_udf(&self, tenant: &str, udf_name: &str) -> Result<bool> {
        let res = self.get_udf(tenant, udf_name).await?;
        Ok(res.is_some())
    }

    // Get all UDFs for the tenant.
    #[async_backtrace::framed]
    pub async fn get_udfs(&self, tenant: &str) -> Result<Vec<UserDefinedFunction>> {
        let udf_api = self.for_tenant(tenant)?.udf_api();

        let udfs = udf_api
            .list_udf()
            .await
            .map_err(|e| e.append_context("while get UDFs"))?;

        Ok(udfs)
    }

    // Drop a UDF by name.
    #[async_backtrace::framed]
    pub async fn drop_udf(
        &self,
        tenant: &str,
        udf_name: &str,
        allow_no_change: bool,
    ) -> std::result::Result<std::result::Result<(), UdfError>, UdfApiError> {
        let dropped = self
            .for_tenant(tenant)?
            .udf_api()
            .drop_udf(udf_name, MatchSeq::GE(1))
            .await?;

        let drop_result = if dropped.is_none() {
            if allow_no_change {
                Ok(())
            } else {
                Err(UdfError::NotFound {
                    tenant: tenant.to_string(),
                    name: udf_name.to_string(),
                    context: "while drop_udf".to_string(),
                })
            }
        } else {
            Ok(())
        };

        Ok(drop_result)
    }
}
