
<p align="center" width="100%" backround-color="red">
    <img src="./images/cachew-logo.png" width="500">
</p>

## A light weight, strongly typed, in-memory, ordered, key-value database.
###### ⚠️ CachewDB is still in development!

### :memo: Features:
- cached / in-memory, non-persistant -> therefore fast
- uses a b-tree map to store data ordered by keys
- same type for all database values
- simple password authentication
- custom socket protocol for communication (_CASP_: Cashew Socket Protocol)
- graceful shutdown

### :memo: Running CachewDB:
###### CachewDB is not ready for production and hasn't even got an implemented client, but you can play around with it anyways using netcat.

##### 1. Clone this repository.
```bash
git clone
```

##### 2. Build using Cargo:
```bash
cargo build
```

##### 5. Run using Cargo:
When running CachewDB, you have to specifiy which data type it should store. You can do so by using the ``--db-type`` (short: ``-t``) flag like this:
```bash
cargo run -- --db-type STR     # see the "Types" section below to see all possible types
```
Or, instead you can specify the type using the environment variable ``CACHEW_DB_TYPE``:
```bash
export CACHEW_DB_TYPE="STR"
cargo run
```

The server now runs on ``127.0.0.1:8080``.

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

### :memo: Keys:
Key can consist of any characters expect spaces (" ") or commata (","), unless they key is encapsulated inside quotes. For example ``?939__.`` and ``"key one"`` are a valid keys.

### :memo: Types:
Supported types are:
| type | description |
|:-------|:----------|
| **STR** | Simple string (must be encapsulated with ``"``). |
| **INT** | 32 bit signed integer. |
| **FLOAT** | 32 bit float. |
| **BOOL** | Either ``true`` or ``false``. |
| **JSON** | Behaves the same as strings (must be encapsulated with ``"``). |


### :memo: CASP protocol specification:
For the specification of the custom protocol used for communicating with a CachewDB instance check this document: [CASP_SPECIFICATION.md](./CASP_SPECIFICATION.md)
