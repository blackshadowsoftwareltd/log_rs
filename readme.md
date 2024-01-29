# Log Rust

## Date format from fn

- `_date` `2024-01-29`

- `_date_time` `2024-01-29 14:13:04`

- `_utc_time` `2024-01-29T08:16:51.758199030+00:00`

- `_utc_date_time` `2024-01-29T08:16:51`

- `_local_time` `2024-01-29T08:16:51.758199030+00:00`

- `_local_date_time` `2024-01-29T08:16:51`

### Actual Logs

```log
2024-01-29 20:51:43 [ERROR] [src/main.rs ~~ 8] 
~> This is an error message
```

```log
2024-01-29 20:51:43 [INFO] [src/main.rs ~~ 9] 
~> "This is an info message"
```

```log
2024-01-29 20:51:43 [WARN] [src/main.rs ~~ 10] 
~> "This is a warning message"
```
