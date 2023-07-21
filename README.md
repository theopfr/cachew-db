
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


### :memo: CASP specification:
To send the commands above to a CachewDB, the custom CASP protocol has to be followed:

#### Requests:
| prefix | delimiter | message | delimiter |suffix |
|:-------:|:-------:|:-------:|:-------:|:-------:|
| CASP | / | *command* | / | /\n |

Examples:
- CASP/SET myKey "some value"/\n
- CASP/GET myKey/\n
- CASP/SET MANY k1 1, k2 2, k3 3/\n

#### Responses:
For GET requests:
| prefix | delimiter | status | delimiter | type | delimiter | command | delimiter | message | delimiter | suffix |
|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|
| CASP | / | OK | / | *type* | / | *command* | / | *response* | / | \n |

For any other requests requests (the difference to the GET request is that there is no type paramter in the response):
| prefix | delimiter | status | delimiter | command | delimiter | message | delimiter | suffix |
|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|
| CASP | / | OK | / | *type* | / | *command* | / | *response* | / | \n |

Where *type* is one of: 
**STR**, **INT**, **FLOAT**

Where *command* is one of the command identifiers: 
**AUTH**, **SET**, **SET MANY**, **GET**, **GET MANY**, **GET RANGE**, **DEL**, **DEL MANY**, **DEL RANGE**

Examples:
- CASP/OK/Authentication succeeded./\n
- CASP/OK/SET/\n
- CASP/OK/STR/GET MANY/"v1","v2"/\n
- CASP/ERROR/ParserError 'invalidKeyValuePair': Expected two parameters (key and value), found 1./\n

