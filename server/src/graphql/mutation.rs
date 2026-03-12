use async_graphql::{Context, Object, Result};

use crate::AppState;
use crate::models::eligibility::{EligibilityCheckRequest, EligibilityCheckResponse};
use crate::clients::eligibility_api::EligibilityClient;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Mutates the system or proxies a verifying HTTP mutation call to Stedi/pVerify to verify healthcare eligibility.
    async fn check_eligibility(
        &self,
        ctx: &Context<'_>,
        request: EligibilityCheckRequest,
    ) -> Result<EligibilityCheckResponse> {
        let state = ctx.data::<AppState>().unwrap();
        
        let client = EligibilityClient::new(
            state.config.clone(),
            state.http_client.clone(),
        );

        let result = client.check_eligibility(&request).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }
}
