
## CASP - Cachew Socket Protocol

CASP is the custom protocol used for communicating with a CachewDB instance via the TCP protocol. The following describes how requests to the server and responses from the server are structured (this information is important for creating clients).

---

### :point_up: In general...:
Each request and response starts with the prefix ``CASP`` and ends with a ``\n``. All parts (prefix, status, message, etc.) of a CASP request/response are joined by the dilimiter ``/``.

---

### :arrow_left: Request specification:
#### Structure:
| prefix | delimiter | message | delimiter |suffix |
|:-------:|:-------:|:-------:|:-------:|:-------:|
| CASP | / | *command* | / | /\n |

#### Examples:
- ``CASP/AUTH password/\n``
- ``CASP/SET key "some value"/\n``
- ``CASP/SET MANY k1 1, k2 2, k3 3/\n``
- ``CASP/GET key/\n``
- ``CASP/GET MANY k1 k2 k3/\n``
- ``CASP/GET RANGE k1 k3/\n``
- ``CASP/DEL MANY k1 k2 k3/\n``
- ``CASP/DEL RANGE k1 k3/\n``

---

### :arrow_right: Response specification:
#### Response structure for GET requests:
   | prefix | delim. | status | delim. | type | delim. | cmd. type | delim. | message | delim. | suffix |
   |:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|
   | CASP | / | OK | / | *type* | / | *cmd. type* | / | *response* | / | \n |

#### Response structure for other requests:
   | prefix | delim. | status | delim. | cmd. type | delim. | suffix |
   |:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|
   | CASP | / | OK | / | *cmd. type* | / | \n |

#### Response structure for errors:
   | prefix | delim. | status | delim. | message | delim. | suffix |
   |:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|:-------:|
   | CASP | / | ERROR | / | *error message* | / | \n |

#### Where ...
... *type* is one of: 
``STR``, ``INT``, ``FLOAT``, ``BOOL``, ``JSON``
... *cmd. type* is one of the command identifiers:
``AUTH``, ``SET``, ``GET``, etc.

#### Examples:
- ``CASP/OK/AUTH/Authentication succeeded./\n``
- ``CASP/OK/SET/\n``
- ``CASP/OK/SET MANY/\n``
- ``CASP/OK/DEL/\n``
- ``CASP/OK/DEL MANY/\n``
- ``CASP/OK/DEL RANGE/\n``
- ``CASP/OK/INT/GET/-99/\n``
- ``CASP/OK/STR/GET MANY/"v1","v2""/\n``
- ``CASP/OK/FLOAT/GET RANGE/0.1, 0.5, 1.99/\n``
- ``CASP/ERROR/Some error message!/\n``

---

### :mailbox: Response bodies for GET requests:
As seen before, the value/-s returned from GET / GET MANY / GET RANGE requests are in the following section:
```
CASP/OK/STR/GET/<--HERE-->/\n
```

The following focuses on how to parse the data within the fourth and fifth ``/``.

#### 1. Parsing integers:
A GET request on an integer database promises to return a simple sequence of characters from 0-9 and potentially a minus sign at the beginning which can be parsed into an integer by the client.

Responses on GET MANY / GET RANGE requests return a many integers seperated by a comma.

##### Examples: 
- ``CASP/OK/GET/INT/10/\n`` -> ``10``
- ``CASP/OK/GET MANY/INT/1,2,3/\n`` -> ``1``, ``2``, ``3``
- ``CASP/OK/GET RANGE/INT/3,2,7/\n`` -> ``3``, ``2``, ``7``

#### 2. Parsing floats:
A GET request on a float database promises to return a simple sequence of characters from 0-9 and potentially a minus sign at the beginning and always with a dot ``.`` as the decimal seperator. which can be parsed into a float by the client.

Responses on GET MANY / GET RANGE requests return a many floats seperated by a comma.

##### Examples: 
- ``CASP/OK/GET/FLOAT/-0.1/\n`` -> ``-0.1``
- ``CASP/OK/GET MANY/FLOAT/0.9,10.0,-20.284/\n`` -> ``0.9``, ``10.0``, ``-20.284``
- ``CASP/OK/GET RANGE/FLOAT/0.009,1.25/\n`` -> ``0.009``, ``1.25``

#### 2. Parsing strings:
A GET request on a string database promises to return a simple sequence of any characters encapsuled in ``"`` which can be parsed into a string by the client.

Responses on GET MANY / GET RANGE requests return a many strings seperated by a comma.

##### Examples: 
- ``CASP/OK/GET/STR/"hello"/\n`` -> ``"hello""``
- ``CASP/OK/GET MANY/STR/"a","b","c"/\n`` -> ``"a"``, ``"b"``, ``"c"``
- ``CASP/OK/GET RANGE/STR/"x","r","z","s"/\n`` -> ``"x"``, ``"r"``, ``"z"``, ``"s"``

---

#### ❌ Error responses:
As seen before, the error message returned is in the following section:
```
CASP/ERR/<--HERE-->/\n
```

##### Error types:
| error type | description |
|:-------|:-------|
| AuthenticationError | Errors concerning the authentication. |
| ProtocolError | Errors thrown if the request isn't valid CASP. |
| ParserError | Errors thrown if the request body is invalid. |
| DatabaseError | Errors thrown if a database query fails. |

##### Structure:
All error messages are structured in the following way:
The error type (see above), then a specific error type, then a colon and finally the error descripion.

##### Examples:
- ``AuthenticationError 'notAuthenticated': Please authenticate before executing queries.``
- ``AuthenticationError 'authenticationFailed': Wrong password.``
- ``ProtocolError 'emptyRequest': Can't process empty request.``
- ``ProtocolError 'startMarkerNotFound': Expected request to start with 'CASP/'.``
- ``ProtocolError 'endMarkerNotFound': Expected request to end with '/\n'``
- ``ParserError 'invalidRange': Expected two keys got 3.``
- ``ParserError 'unexpectedCharacter': ',' is not allowed in keys.``
- ``ParserError 'invalidKeyValuePair': Expected two parameters (key and value), found 1.``
- ``ParserError 'unknownQueryOperation': Query 'SER key 10' not recognized.``
- ``ParserError 'wrongValueType': The value doesn't match the database type.``
- ``ParserError 'wrongAuthentication': Couldn't read password. Expecting: 'AUTH <password>'``
- ``DatabaseError 'keyNotFound': The key '2d837e' doesn't exist.``
- ``DatabaseError 'invalidRangeOrder': The lower key is bigger than the upper key.``
- ``DatabaseError 'wrongValueType': The value doesn't match the database type.``
