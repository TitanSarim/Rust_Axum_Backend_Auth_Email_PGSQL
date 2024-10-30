// Import necessary modules and types from `axum` for handling HTTP responses and status codes.
use axum::{
    http::StatusCode,            // HTTP status codes to use for response statuses.
    response::{IntoResponse, Response}, // Traits to convert custom types into HTTP responses.
    Json                         // Allows serializing Rust types into JSON.
};

// Import serialization and deserialization traits from `serde`.
// `Deserialize` allows deserializing JSON into Rust structs, and `Serialize` allows the reverse.
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

// Implement the `Display` trait for `ErrorResponse` to enable easy string formatting.
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Serialize the struct to JSON and write it to the formatter.
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
#[derive(Debug, PartialEq)]
pub enum ErrorMessage{
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

// Implement `ToString` trait for `ErrorMessage` to convert error enums to their string representations.
impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

// Define a method that provides the string message associated with each `ErrorMessage` variant.
impl ErrorMessage{
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
            ErrorMessage::EmailExist => "A user with this email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => "User belonging to this token no longer exists".to_string(),
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => format!("Password must not be more than {} characters", max_length),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide a token".to_string(),
            ErrorMessage::PermissionDenied => "You are not allowed to perform this action".to_string(),
            ErrorMessage::UserNotAuthenticated => "Authentication required. Please log in.".to_string(),
        }
    }
}

// Struct to represent HTTP errors with status code and message for detailed response handling.
#[derive(Debug, Clone)]
pub struct HttpError{
    pub message: String,
    pub status: StatusCode,
}

// Implement constructor and utility functions for `HttpError` for easy instantiation of common errors.
impl HttpError {
    // General constructor for `HttpError` with customizable message and status code.
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError{
            message: message.into(),
            status
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError{
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError{
            message: message.into(),
            status: StatusCode::BAD_REQUEST
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError{
            message: message.into(),
            status: StatusCode::CONFLICT
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError{
            message: message.into(),
            status: StatusCode::UNAUTHORIZED
        }
    }

    // Converts `HttpError` to a `Response` by creating a JSON payload.
    pub fn into_http_response(self) -> Response{
        // Define JSON response containing a `fail` status and the error message.
        let json_response = Json(ErrorResponse{
            status: "fail".to_string(),
            message: self.message.clone(),
        });
        // Combine the HTTP status and JSON response into an `IntoResponse`-compatible tuple.
        (self.status, json_response).into_response()
    }
}

// Implement `Display` for `HttpError` to allow string formatting and easy printing.
impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}

// Implement `std::error::Error` trait for `HttpError`, making it compatible with error handling frameworks.
impl std::error::Error for HttpError {}

// Implement `IntoResponse` for `HttpError` so it can be used directly in axum handler return types.
impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}