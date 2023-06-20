# Recommender

Recommends meals via email.

## Running locally

```bash
cargo lambda watch

# in a separate terminal window
cargo lambda invoke --data-ascii '{"time": "2023-06-16 23:59:35.216721974 UTC", "resources": []}'
```
