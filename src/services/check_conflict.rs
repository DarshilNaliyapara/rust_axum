use std::collections::HashMap;
use diesel::sql_types::Text;
use diesel_async::{RunQueryDsl};
use crate::error::AppError;

#[derive(diesel::QueryableByName)]
struct ExistsResult {
    #[diesel(sql_type = diesel::sql_types::Bool)]
    exists: bool,
}

pub async fn check_conflicts(
    conn: &mut diesel_async::AsyncPgConnection,
    table_name: &str,
    fields: HashMap<String, String>,
    exclude_id: Option<uuid::Uuid>, // The key to flexibility
) -> Result<(), AppError> {
    let mut conflicts = Vec::new();

    for (column_name, value) in fields {
        // Base query
        let mut query = format!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1", 
            table_name, column_name
        );
        
        if let Some(id) = exclude_id {
            query.push_str(&format!(" AND id != '{}'", id));
        }
        
        query.push_str(") AS exists");

        let result: ExistsResult = diesel::sql_query(query)
            .bind::<Text, _>(value)
            .get_result::<ExistsResult>(conn)
            .await?;

        if result.exists {
            conflicts.push(column_name);
        }
    }

    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(AppError::Conflict(conflicts))
    }
}
