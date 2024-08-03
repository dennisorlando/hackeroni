use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable)]
#[diesel(table_name = super::schema::users)]
pub struct NewUser {
    pub name: String,
    pub password_hash: String,
}

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = super::schema::users)]
pub struct UserMem {
    pub id: i64,
    pub name: String,
    pub password_hash: String,
    pub is_admin: bool,
}
type DbError = diesel::result::Error;
pub fn insert_new_user(conn: &mut PgConnection, new_user: NewUser) -> Result<UserMem, DbError> {
    use super::schema::users::dsl::*;

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)?;

    Ok(user)
}
pub fn get_user_by_name(conn: &mut PgConnection, username: &str) -> Result<UserMem, DbError> {
    use super::schema::users::dsl::*;

    let user: UserMem = users.filter(name.eq(username)).get_result(conn)?;

    Ok(user)
}


