# API Reference

The `0.1.0` API defines the foundation system endpoints. The secret-storage
operations are intentionally not implemented until later releases.

## `GET /v1/sys/health`

Returns process health and foundation seal state.

```bash
curl -s http://127.0.0.1:8200/v1/sys/health
```

Example response:

```json
{
  "initialized": false,
  "sealed": true,
  "version": "0.1.0"
}
```

## `GET /v1/sys/seal-status`

Returns initialization and seal progress.

```bash
curl -s http://127.0.0.1:8200/v1/sys/seal-status
```

Example response:

```json
{
  "initialized": false,
  "sealed": true,
  "threshold": 0,
  "progress": 0
}
```

## `GET /v1/sys/version`

Returns the Lykilheim version.

```bash
curl -s http://127.0.0.1:8200/v1/sys/version
```

Example response:

```json
{
  "version": "0.1.0"
}
```

## `POST /v1/sys/init`

Defined in `0.1.0`, implemented in `0.2.0`.

```bash
curl -s -X POST http://127.0.0.1:8200/v1/sys/init
```

Example `0.1.0` response:

```json
{
  "code": "not_implemented",
  "message": "sys/init is defined for 0.1.0 but implemented in 0.2.0"
}
```

## `POST /v1/sys/unseal`

Defined in `0.1.0`, implemented in `0.2.0`.

```bash
curl -s -X POST http://127.0.0.1:8200/v1/sys/unseal
```

## `POST /v1/sys/seal`

Defined in `0.1.0`, implemented in `0.2.0`.

```bash
curl -s -X POST http://127.0.0.1:8200/v1/sys/seal
```

## Error Format

Errors return a stable JSON object:

```json
{
  "code": "not_implemented",
  "message": "sys/init is defined for 0.1.0 but implemented in 0.2.0"
}
```
