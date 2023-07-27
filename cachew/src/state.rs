use std::collections::{HashMap};
use std::time::{Duration};
use tokio::sync::broadcast;

use crate::auth_error;
use crate::schemas::{DatabaseType, QueryRequest, QueryResponseType};
use crate::database::Database;
use crate::errors::authentication_errors::{AuthenticationErrorType};



pub struct State {
    pub db: Database,
    pub auth_table: HashMap<String, bool>,
    pub password: String,
    pub database_type: DatabaseType,
    pub shutdown_tx: broadcast::Sender<()>,
    pub shutdown_rx: broadcast::Receiver<()>
}

impl State {
    pub fn new(database_type: DatabaseType, password: String) -> Self {
        let db: Database =  Database::new(database_type);
        let auth_table: HashMap<String, bool> = HashMap::new();

        let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(1);

        Self {
            db,
            auth_table,
            password,
            database_type,
            shutdown_tx,
            shutdown_rx
        }
    }

    pub fn is_authenticated(&self, address: String) -> bool {
        matches!(self.auth_table.get(&address), Some(_))
    }

    pub fn deauthenticate(&mut self, address: &str) {
        self.auth_table.remove(address);
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
            QueryRequest::AUTH(password) => self.authenticate(address, &password),
            QueryRequest::CLEAR => self.db.clear(),
            QueryRequest::LEN => self.db.len(),
            QueryRequest::PING => Ok(QueryResponseType::PING_OK),
            QueryRequest::EXISTS(key) => self.db.exists(&key),
            QueryRequest::SHUTDOWN => Ok(QueryResponseType::SHUTDOWN_OK)
        }
    }

    pub async fn signal_shutdown(&self) {
        let _ = self.shutdown_tx.send(());
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    pub fn subscribe_shutdown_channel(&self) -> broadcast::Receiver<()> {
        self.shutdown_rx.resubscribe()
    }
}


#[cfg(test)]
mod tests {
    use crate::schemas::{KeyValuePair, ValueType};
    use super::*;

    #[test]
    fn test_authentication() {
        let database_type = DatabaseType::Str;
        let mut state: State = State::new(database_type, "pwd123".to_string());

        let client_address: &str = "0.0.0.0:0000";

        let authenticated = state.authenticate(client_address, "pwd123");
        assert_eq!(authenticated, Ok(QueryResponseType::AUTH_OK));

        let is_authenticated = state.is_authenticated(client_address.to_owned());
        assert!(is_authenticated);

        state.deauthenticate(client_address);
        assert!(!matches!(state.auth_table.get(client_address), Some(_)));

        let failed_authentication = state.authenticate(client_address, "wrongpassword");
        assert_eq!(failed_authentication.unwrap_err(), "AuthenticationError 'authenticationFailed': Wrong password.");
    }

    #[test]
    fn test_execute_query() {
        let database_type = DatabaseType::Str;
        let mut state: State = State::new(database_type, "pwd123".to_string());

        let client_address: &str = "0.0.0.0:0000";

        // request without being authenticated
        let not_authenticated = state.execute_request(client_address, QueryRequest::SET(KeyValuePair { key: "key".to_string(), value: ValueType::Str("value".to_string()) }));
        assert_eq!(not_authenticated.unwrap_err(), "AuthenticationError 'notAuthenticated': Please authenticate before executing queries.");

        // make authentication request        
        let authentication_request = state.execute_request(client_address, QueryRequest::AUTH("pwd123".to_string()));
        assert_eq!(authentication_request, Ok(QueryResponseType::AUTH_OK));

        // test query requests

        let response_set = state.execute_request(client_address, QueryRequest::SET(KeyValuePair { key: "key".to_string(), value: ValueType::Str("value".to_string()) }));
        assert_eq!(response_set, Ok(QueryResponseType::SET_OK));

        let response_set_many = state.execute_request(client_address, QueryRequest::SET_MANY(vec![
            KeyValuePair { key: "key1".to_string(), value: ValueType::Str("value1".to_string()) },
            KeyValuePair { key: "key2".to_string(), value: ValueType::Str("value2".to_string()) },
            KeyValuePair { key: "key3".to_string(), value: ValueType::Str("value3".to_string()) },
            KeyValuePair { key: "key4".to_string(), value: ValueType::Str("value4".to_string()) },
            KeyValuePair { key: "key5".to_string(), value: ValueType::Str("value5".to_string()) }
        ]));
        assert_eq!(response_set_many, Ok(QueryResponseType::SET_MANY_OK));

        let response_get = state.execute_request(client_address, QueryRequest::GET("key1".to_string()));
        assert_eq!(response_get, Ok(QueryResponseType::GET_OK(ValueType::Str("value1".to_string()))));

        let response_get_many = state.execute_request(client_address, QueryRequest::GET_MANY(vec!["key3", "key2"]));
        assert_eq!(response_get_many, Ok(QueryResponseType::GET_MANY_OK(vec![ValueType::Str("value3".to_string()), ValueType::Str("value2".to_string())])));

        let response_get_range = state.execute_request(client_address, QueryRequest::GET_RANGE { key_lower: "key2".to_string(), key_upper: "key4".to_string() });
        assert_eq!(response_get_range, Ok(QueryResponseType::GET_RANGE_OK(vec![
            ValueType::Str("value2".to_string()), ValueType::Str("value3".to_string()), ValueType::Str("value4".to_string())
        ])));

        let response_ping = state.execute_request(client_address,QueryRequest::EXISTS("key2".to_owned()));
        assert_eq!(response_ping, Ok(QueryResponseType::EXISTS_OK(true)));
        
        let response_del = state.execute_request(client_address, QueryRequest::DEL("key1".to_string()));
        assert_eq!(response_del, Ok(QueryResponseType::DEL_OK));

        let response_del_many = state.execute_request(client_address, QueryRequest::DEL_MANY(vec!["key4", "key3"]));
        assert_eq!(response_del_many, Ok(QueryResponseType::DEL_MANY_OK));

        let response_del_range = state.execute_request(client_address, QueryRequest::DEL_RANGE { key_lower: "key2".to_string(), key_upper: "key5".to_string() });
        assert_eq!(response_del_range, Ok(QueryResponseType::DEL_RANGE_OK));    
        
        let response_clear = state.execute_request(client_address, QueryRequest::CLEAR);
        assert_eq!(response_clear, Ok(QueryResponseType::CLEAR_OK));

        let response_len = state.execute_request(client_address, QueryRequest::LEN);
        assert_eq!(response_len, Ok(QueryResponseType::LEN_OK(0)));

        let response_ping = state.execute_request(client_address,QueryRequest::PING);
        assert_eq!(response_ping, Ok(QueryResponseType::PING_OK));
    }

}

