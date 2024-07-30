use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable)]
#[diesel(table_name = super::schema::users)]
struct NewUser {
    name: String,
}

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = super::schema::users)]
pub struct User {
    pub id: i64,
    pub name: String,
}
type DbError = Box<dyn std::error::Error + Send + Sync>;
pub fn insert_new_user(conn: &mut PgConnection, user_name: String) -> Result<User, DbError> {
    use super::schema::users::dsl::*;

    // Create insertion model
    let new_user = NewUser {
        name: user_name,
    };

    let user = diesel::insert_into(users).values(&new_user).get_result(conn)?;

    Ok(user)
}
