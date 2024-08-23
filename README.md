# lkn Protocol

## 1. Overview

LKN is a simple protocol designed for learning purposes, inspired by HTTP but with a unique twist: responses can be randomly altered before reaching the client, based on client preference.

## 2. Connection

- Uses TCP on port 6969
- No special handshake required

## 3. Message Format

### 3.1 Request Format

```
LKN-METHOD /resource
Header1: value1
Header2: value2
LKN-Randomize: [true/false]

[Optional Body]

```

### 3.2 Response Format

```
LKN STATUS_CODE
Content-Type: text/plain
LKN-Randomized: [true/false]

[Response Body]

```

## 4. Methods

- GET: Retrieve a resource
- POST: Submit data to be processed

## 5. Headers

- Content-Type: MIME type of the response body
- LKN-Randomize: (Request) Indicates if the client wants response randomization
- LKN-Randomized: (Response) Indicates if the response was randomized

## 6. Status Codes

- 200: OK
- 400: Bad Request
- 404: Not Found
- 500: Internal Server Error

## 7. Response Randomization

The server applies randomization to the response body before sending it to the client only if the client requests it via the LKN-Randomize header.
Randomization method: Replace 30% of characters with random ASCII characters.

## 8. Example Exchange

Request:

```
GET /hello
LKN-Randomize: true

```

Response (after randomization):

```
LKN 200
Content-Type: text/plain
LKN-Randomized: true

He%lo, W@rld!
```
