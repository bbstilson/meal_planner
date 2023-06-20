# Responder

Responds to requests to plan meals. Creates the ingredients and populates them into the grocery list.

## Running locally

```bash
cargo lambda watch

# in a separate terminal window
cargo lambda invoke meal-planner-responder --data-file request_examples/basic.json
```
