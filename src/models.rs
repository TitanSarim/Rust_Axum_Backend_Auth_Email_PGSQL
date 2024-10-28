use chrono::prelude::*; // Provides date and time functionalities
use serde::{Deserialize, Serialize};// Provides serialization and deserialization capabilities

// Defining an enum called UserRole to represent user roles, which can be either Admin or User
#[derive(Serialize, Deserialize, Debug, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")] // Tells sqlx to use the lowercase string representation for database mapping
pub enum UserRole {
    Admin,
    User,
}

// Implementing methods for UserRole enum
impl UserRole{
    // Defining a function that returns a string representation of the UserRole

    // ! &self is a reference to the current instance of the UserRole enum. It allows the method to access the instance data without taking ownership.
    // ! By convention, &self is used for methods that don’t need to mutate or take ownership of the data.
    // ! Here, it means to_str only reads the data from the instance and doesn’t alter it.
    // ! -> &'static str
    // This part is the function's return type. It means that the function returns a &'static str, which is a reference to a string slice with a 'static lifetime.
    pub fn to_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user"
        }
    }
}

// Defining a struct to represent a User with various fields
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, sqlx::Type)]
pub struct User{
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub verified: bool,
    pub verification_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(rename="createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime<Utc>,
}