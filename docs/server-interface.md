# Server Interface

It would be good to support standard SQL interface, but I also think a more RESTful interface would help with programmatic integrations. I dont have data-points on either, but lets just start with these two anyways.

## RESTful Interface

RESTful interface will be recommended and primary way to interface with the server.

Operations:

| Route                 | Method | Description                      |
|-----------------------|--------|----------------------------------|
| `/tables/<tablename>` | POST   | Create a new table               |
| `/tables/<tablename>` | PUT    | Update an existing table         |
| `/tables/<tablename>` | DELETE | Delete an existing table         |
| `/<tablename>/create` | POST   | Insert rows into a table         |
| `/<tablename>/update` | POST   | Update rows matching a condition |
| `/<tablename>/delete` | POST   | Delete rows matching a condition |
| `/<tablename>/query`  | POST   | Query Rows                       |


<div name="standard-errors">

### Standard Errors

**Client Error**

```
400 BAD_REQUEST | 404 NOT_FOUND | 408 REQUEST_TIMEOUT | 429 THROTTLED

{
    "error_code": <ENUM>,
    "error_message: "<Human Readable Error Message>"
}
```

**Server Error**

```
500 INTERNAL_SERVER_ERROR

{
    "error_code": <ENUM>,
    "error_message: "<Human Readable Error Message>"
}
```

**Error Codes**

| Error Code           | Description                                     | HTTP Status Code |
|----------------------|-------------------------------------------------|------------------|
| bad_schema           | Provided schema is bad                          | 400 BAD_REQUEST  |
| no_such_table        | Table does not exist                            | 400 BAD_REQUEST  |
| column_not_in_schema | Column not defined in schema                    | 400 BAD_REQUEST  |
| column_type_mismatch | Column has different type defined in schema     | 400 BAD_REQUEST  |
| invalid_json_body    | Json Body provided is invalid for the operation | 400 BAD_REQUEST  |


</div>



### Create a new table

**Request**

```http
POST /tables/<tablename>

{
    "schema": {
        "columnname1": "datatype",
        "columnname2": "datatype",
        ...
    },
    "settings": {
        "strict_schema": boolean
    }
}
```

**Response**

```http
200 OK

{
    "table": "<tablename>",
    "schema": {
        "columnname1": "datatype",
        "columnname2": "datatype",
    },
    "settings": {
        ...
    }
}
```

For errors check [Standard Errors](#standard-errors)


### Update an existing table

TBA


### Delete an existing table


**Request**

```http

DELETE /tables/<tablename>

```


**Response**

```http
200 OK

{
    "table": "<tablename>",
    "schema": {
        "columnname1": "datatype",
        "columnname2": "datatype",
    },
    "settings": {
        ...
    }
}
```

For errors check [Standard Errors](#standard-errors)


### Insert rows into a table

**Request**

```http

POST /<tablename>/create

{
    "rows": [
        {
            "columnname1": "value",
            "columnname2": "value",
            ...
        },
        ...
    ]
}

```


**Response**

```http
200 OK

{
    "table": "<tablename>",
    "rows_created": <n>
}
```

For errors check [Standard Errors](#standard-errors)


### Update rows matching a condition

TBA

### Delete rows matching a condition

TBA

### Query rows

TBA


## SQL Interface

We support SQL Interface as secondary option on top of RESTful interface. Feature parity between both is not guranteed and RESTful interface can have more features.

DBs usually have different ports to support different protocols. Should we expose SQL interface as different protocol or wrap it in JSON and use HTTP protocol?

It would be good to look into pros/cons. But for now I will keep everything wrapped in JSON + HTTP. We can come back and switch it later.

Operations to support:

| Operation    | Description                      |
|--------------|----------------------------------|
| CREATE TABLE | Create a new table               |
| ALTER TABLE  | Update an existing table         |
| DROP TABLE   | Delete an existing table         |
| INSERT INTO  | Insert rows into a table         |
| UPDATE       | Update rows matching a condition |
| DELETE       | Delete rows matching a condition |
| SELECT       | Select rows matching a condition |

### CREATE TABLE

```sql
CREATE TABLE table_name (
    column1 datatype,
    column2 datatype,
    ...
);
```

### ALTER TABLE

```sql
ALTER TABLE table_name
ADD column_name datatype;

ALTER TABLE table_name
DROP COLUMN column_name;

ALTER TABLE table_name
RENAME COLUMN old_name to new_name;

ALTER TABLE table_name
MODIFY COLUMN column_name datatype;
```

### DELETE TABLE 

```sql
DROP TABLE table_name;
```


### INSERT INTO

```sql
INSERT INTO table_name (column1, column2, column3, ...)
VALUES (value1, value2, value3, ...);
```

### UPDATE

```sql
UPDATE table_name
SET column1 = value1, column2 = value2, ...
WHERE condition;
```

### DELETE

```sql
DELETE FROM table_name WHERE condition;
```


### SELECT

```sql
SELECT *
FROM table_name
WHERE condition;
```

`SELECT` can be much more complicated, so we will cover it seperately.

