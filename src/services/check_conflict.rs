use std::collections::HashMap;
use diesel::sql_types::Text;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::error::AppError;

#[derive(diesel::QueryableByName)]
struct ExistsResult {
    #[diesel(sql_type = diesel::sql_types::Bool)]
    exists: bool,
}

pub async fn check_conflicts(
    conn: &mut AsyncPgConnection,
    table_name: &str,
    fields: HashMap<String, String>,
) -> Result<(), AppError> {
    let mut conflicts = Vec::new();

    for (column_name, value) in fields {
        let query = format!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1) AS exists",
            table_name, column_name
        );

        let result: ExistsResult = diesel::sql_query(query)
            .bind::<Text, _>(value)
            .get_result(conn)
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
