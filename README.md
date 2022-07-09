# MAX CAPACITY

Imagine a scenario where you write some code with pre-defined allocation sizes. You've done your calculations, and your vec or hash map *should* never exceed that capacity threshold. *But does it?*.

The `max_capacity` crate helps you validate that assertion at runtime either by panicing or logging when capacity is exceeded:

```rust
Example here
```
