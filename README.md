# pg_rfc8785

PostgreSQL extension that canonicalizes JSON text according to RFC 8785 / JCS.

The function accepts `text` intentionally. For `jsonb`, call it with `payload::text`:

```sql
SELECT rfc8785_canonicalize('{"b":2,"a":1}'::jsonb::text);
-- {"a":1,"b":2}
```

## Functions

```sql
rfc8785_canonicalize(input text) returns text
rfc8785_is_canonical(input text) returns boolean
```

Both are marked `IMMUTABLE`, `STRICT`, and `PARALLEL SAFE`.

## Build into the pinned TimescaleDB HA PG18 runtime

```bash
docker build \
  -f Dockerfile.build-extension \
  -t timescaledb-ha-pg18-rfc8785:pg18.0-ts2.23.0-pg_rfc8785-0.1.0 \
  .
```

Run:

```bash
docker run -it --rm \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  timescaledb-ha-pg18-rfc8785:pg18.0-ts2.23.0-pg_rfc8785-0.1.0
```

Use:

```sql
CREATE EXTENSION pg_rfc8785;

SELECT rfc8785_canonicalize('{"b":2,"a":1}'::jsonb::text);
```

With pgcrypto hashing:

```sql
CREATE EXTENSION pgcrypto;

SELECT encode(
  digest(rfc8785_canonicalize('{"b":2,"a":1}'::jsonb::text), 'sha1'),
  'hex'
);
```

## Notes

RFC 8785 / JCS is intended to create a stable, hashable JSON representation.
The underlying `serde_json_canonicalizer` crate notes that arbitrary precision numbers are converted into doubles for canonical serialization, so numeric edge cases should be tested for your content-addressed object IDs.
