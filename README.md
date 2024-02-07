
[![CI cachew server](https://github.com/theopfr/cachew-db/actions/workflows/ci_server.yml/badge.svg)](https://github.com/theopfr/cachew-db/actions/workflows/ci_server.yml)
&nbsp;&nbsp;&nbsp;
[![CI cli-client](https://github.com/theopfr/cachew-db/actions/workflows/ci_cli_client.yml/badge.svg)](https://github.com/theopfr/cachew-db/actions/workflows/ci_cli_client.yml)

<p align="center" width="100%" backround-color="red">
    <img src="./images/cachew-logo.png" width="500">
</p>

## A light weight, typed, in-memory, ordered, key-value database.

### :memo: Features:
- cached / in-memory, non-persistant
- uses a b-tree map to store data ordered by keys
- same type for all database values
- simple password authentication
- custom socket protocol for communication (_CASP_: Cashew Socket Protocol)
- graceful shutdown
- concurrent client handling

---

### :memo: Running the CachewDB server:
##### 1. Build using Cargo:
```bash
cd ./cachew/
cargo build --release
```

##### 2. Run using Cargo:
To start the CachewDB server you have to run ``cargo run`` or the binary ``./target/release/cachew``. You also have to specify the following arguments either with command line flags or using environment variables:

| flag | short | description | ENV |
|:-------|:----------|:----------|:----------|
| --db-type | -t | Sets the data type (check possible types in the "Types" section). | CACHEW_DB_TYPE |
| --password | -p | Sets the password for the database (must have at least: 1 upper-, 1 lowercase letter, 1 special char., >= 8 chars.). | CACHEW_DB_PASSWORD |
| --host | n/a | The host address the server will be running on (optional, default: ``127.0.0.1``). | CACHEW_DB_HOST |
| --db-type | n/a | The port the server will be accessible on (optional, default: ``8080``). | CACHEW_DB_PORT |

##### Examples:
1. Using flags to set the db-type and password.
   ```bash
   cargo run --release -- -t STR -p Password123#     # host will default to 127.0.0.1 and port to 8080
   ```
2. Using ENV variables to specify all arguments:
   ```bash
   export CACHEW_DB_TYPE="STR"
   export CACHEW_DB_PASSWORD="Password123#"
   export CACHEW_DB_HOST="127.0.0.1"
   export CACHEW_DB_PORT="2345"
   cargo run --release
   ```

---

### :memo: Running the CachewDB CLI client:

##### 1. Build using Cargo:
```bash
cd ./cli-client/
cargo build --release
```

##### 2. Run using Cargo:
To start the CachewDB CLI client you have to run ``cargo run`` or the binary ``./target/release/cli-client``. You have to specify the following arguments with command line flags:

| flag | description |
|:-------|:----------|
| --host | Sets host remote address of the CacheDB server. |
| --port | Sets the remote port of the CachewDB server. | 
| --password | Sets the password needed for authenticating on the CachewDB server. |

##### Example:
```
cargo run --release -- --host 127.0.0.1 --port 8080 --password Password123#
```

---

### :memo: Commands:
| command | description | example |
|:-------|:----------|:-------|
| **AUTH** {password} | Authentication for a CachewDB instance. | AUTH mypwd123 |
| **SET** {key} {value} | Insert new key value pair. | SET myKey "myValue" |
| **SET MANY** {key} {value}, {key} {value} | Bulk insert multiple key value pairs. | SET MANY key1 "value 1", key2 "value 2" |
| **GET** {key} | Get value from key. | GET myKey |
| **GET MANY** {key} {key} ... | Get multiple values from their keys. | GET MANY key1 key2 key3 |
| **GET RANGE** {lower-key} {upper-key} | Get values from a range of keys. | GET RANGE aKey zKey |
| **DEL** {key} | Delete key value pair. | DEL myKey |
| **DEL MANY** {key} {key} ... | Delete multiple key value pairs. | DEL MANY key1 key2 key3 |
| **DEL RANGE** {lower-key} {upper-key} | Delete a range of key value pairs. | DEL RANGE aKey zKey |
| **CLEAR** | Removes all entries in the database. | CLEAR |
| **LEN** | Returns the amount of entries in the database.| LEN |
| **EXISTS** {key} | Returns a bool signaling if a key exists in the database. | EXSITS key |
| **PING** | Answers with "PONG" (used to check if the server is running). | PING |
| **SHUTDOWN** | Gracefully shutdowns the database. | PING |
---

### :memo: Keys:
A key can consist of every character with two exepctions:
1. If a key should contain a space, comma (``,``) or slash (``/``) it must be encapsuled by quotes (``"``).
2. If a key should contain quotes, the quotes must be escaped by adding a backslash before them (``\"``).

These restrictions exist to make parsing requests and responses easier.

##### Examples of valid keys:
- ``id5ffh26a``
- ``?9%39__``
- ``"my key / 1,2"``
- ``id\"5ffh26a\"``

##### Examples of invalid keys:
- ``id 5ffh26a``
- ``id/5ffh26a``
- ``id"5ffh26a"``
- ``id,5ffh26a``

---

### :memo: Types:
Supported types are:
| type | description |
|:-------|:----------|
| **STR** | Simple string (must be encapsulated with ``"`` and double quotes inside must me escaped like this: ``\"``). |
| **INT** | 32 bit signed integer. |
| **FLOAT** | 32 bit float. |
| **BOOL** | Either ``true`` or ``false``. |
| **JSON** | Behaves the same as strings (must be encapsulated with ``"`` and double quotes inside must me escaped like this: ``\"``). |

---

### :memo: CASP protocol specification:
For the specification of the custom protocol used for communicating with a CachewDB instance check this document: [CASP_SPECIFICATION.md](./CASP_SPECIFICATION.md)

---

#### Todo
- [x] add graceful shutdown
- [x] add command line flag handler
- [x] built CLI client
- [ ] add persistance
- [ ] add INCR/DECR commands for INT type
