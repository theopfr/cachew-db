
<p align="center" width="100%" backround-color="red">
    <img src="./images/cachew-logo.png" width="500">
</p>

## A light weight, strongly typed, in-memory, ordered, key-value database.
###### ⚠️ CachewDB is still in development!

### :memo: Features:
- in-memory, non-persistant -> therefore fast
- uses a B-Tree data structure to store data ordered by keys
- allows only one value type per instance
- simple password authentication
- custom socket protocol for communication (_CASP_: Cashew Socket Protocol)

### :memo: Commands:
| command | description | example |
|:-------|:----------|:-------|
| **AUTH** {password} | Authentication for a CachewDB instance. | *AUTH mypwd123* |
| **SET** {key} {value} | Insert new key value pair. | *SET myKey myValue* |
| **SET MANY** {key} {value}, {key} {value} | Bulk insert multiple key value pairs. | *SET MANY key1 "value 1", key2 "value 2"* |
| **GET** {key} | Get value from key. | *GET myKey* |
| **GET MANY** {key} {key} ... | Get multiple values from their keys. | *GET MANY key1 key2 key3* |
| **GET RANGE** {lower-key} {upper-key} | Get values from a range of keys. | *GET RANGE aKey zKey* |
| **DEL** {key} | Delete key value pair. | *DEL myKey* |
| **DEL MANY** {key} {key} ... | Delete multiple key value pairs. | *DEL MANY key1 key2 key3* |
| **DEL RANGE** {lower-key} {upper-key} | Delete a range of key value pairs. | *DEL RANGE aKey zKey* |

### :memo: Types:
Supported types are:
| type | description |
|:-------|:----------|
| **STR** | Simple string (must be encapsulated with ``"`` when there are spaces). |
| **INT** | 32 bit signed integer. |
| **FLOAT** | 32 bit float. |
| **BOOL** | *todo* |
| **JSON** | *todo* |


### :memo: CASP protocol specification:
For the specification of the custom protocol used for communicating with a CachewDB instance check this document: [CASP_SPECIFICATION.md](./CASP_SPECIFICATION.md)
