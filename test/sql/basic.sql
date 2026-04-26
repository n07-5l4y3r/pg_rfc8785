CREATE EXTENSION pg_rfc8785;

SELECT rfc8785_canonicalize('{"b":2,"a":1}');
SELECT rfc8785_canonicalize('{"z":[3,2,1],"a":{"b":true,"a":null}}');
SELECT rfc8785_is_canonical('{"a":1,"b":2}');
SELECT rfc8785_is_canonical('{"b":2,"a":1}');
SELECT rfc8785_canonicalize('{"b":2,"a":1}'::jsonb::text);
