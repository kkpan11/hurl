GET http://localhost:8000/utf8
HTTP 200
[Asserts]
jsonpath "$.url" urlQueryParam ?? == 1 