use std::collections::HashMap;
use std::net::SocketAddr;

use crate::auth_error;
use crate::schemas::{DatabaseType, QueryRequest, QueryResponseType};
use crate::database::Database;
use crate::errors::authentication_errors::{AuthenticationErrorType};



pub struct State {
    pub db: Database,
    pub auth_table: HashMap<String, bool>,
    pub password: String,
    pub database_type: DatabaseType
}

impl State {
    pub fn new(database_type: DatabaseType, password: String) -> Self {
        let db: Database =  Database::new(database_type);
        let auth_table: HashMap<String, bool> = HashMap::new();

        Self {
            db,
            auth_table,
            password,
            database_type
        }
    }

    pub fn remove_client(&mut self, address: &str) {
        self.auth_table.remove(address);
    }

    pub fn is_authenticated(&self, address: String) -> bool {
        matches!(self.auth_table.get(&address), Some(_))
    }

    pub fn authenticate(&mut self, address: &str, given_password: &str) -> Result<QueryResponseType, String> {
        // if the passwords match, add the client to the auth table
        if given_password == self.password {
            self.auth_table.insert(address.to_owned(), true);

            return Ok(QueryResponseType::AUTH_OK);
        }
        auth_error!(AuthenticationErrorType::AuthenticationFailed)
    }

    pub fn execute_request(&mut self, address: &str, request: QueryRequest) -> Result<QueryResponseType, String> {

        // before executing the query, check if the client address is authenticated
        if !self.is_authenticated(address.to_owned()) {

            // it not, allow to authenticate
            if let QueryRequest::AUTH(password) = request {
                return self.authenticate(address, &password);
            }
            // but disallow other requests
            else {
                auth_error!(AuthenticationErrorType::NotAuthenticated)
            }
        }

        match request {
            QueryRequest::GET(key) => self.db.get(&key),
            QueryRequest::GET_RANGE { key_lower, key_upper } => self.db.get_range(key_lower, key_upper),
            QueryRequest::GET_MANY(keys) => self.db.get_many(keys),
            QueryRequest::DEL(key) => self.db.del(&key),
            QueryRequest::DEL_RANGE { key_lower, key_upper } => self.db.del_range(key_lower, key_upper),
            QueryRequest::DEL_MANY(keys) => self.db.del_many(keys),
            QueryRequest::SET(key_value_pair) => self.db.set(&key_value_pair.key, key_value_pair.value),
            QueryRequest::SET_MANY(key_value_pairs) => self.db.set_many(key_value_pairs),
            QueryRequest::AUTH(password) => self.authenticate(address, &password)
        }
    }
}