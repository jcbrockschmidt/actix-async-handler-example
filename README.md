# Actix Asynchronous Handlers Example

This project provides a working example of how to asynchronously send messages to multiple actors within a single message handler.

## Details

This solution is implemented for `ArbiterService`s, not normal `Actor`s. The core of the solution is within the handler in `src/actor1.rs`. The constructed future is executed regardless of whether the end-user `await`s the result. Within `src/main.rs` we demonstrate how to `await` the result if desired.
