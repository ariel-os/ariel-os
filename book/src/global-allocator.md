# Global Allocator

Ariel OS provides an optional, disabled-by-default [global heap allocator][global-alloc-alloc-std-docs], which can be enabled with the `alloc` [laze module][laze-modules-book].

Using a heap allocator is usually avoided in embedded applications since, by nature, it introduces memory fragmentation and results in runtime errors when running out of memory.
Therefore it should only be enabled when actually necessary.
Use cases include:

- Using a dependency targeting embedded explicitly requiring a dynamic allocator
- Using a [`no_std`][no-std-attr-rust], but not no-`alloc` (transitive) dependency that has not been written with embedded in mind
- Experimenting with [`alloc`][alloc-std-docs]-provided types (e.g., [`Vec<T>`][vec-alloc-std-docs])

Some functionality of Ariel OS may depend on a heap allocator, in which case it is automatically enabled.

## Using the `alloc` Crate

The global allocator allows using the [`alloc` crate][alloc-std-docs] (part of the standard library).
It can be imported using the following:

```rust
extern crate alloc;
```

Its items can then be imported as usual.

> [!TIP]
> When possible, consider using the [`heapless` crate][heapless-docsrs] instead, which provides similar types without dynamic allocation, by statically allocating a memory pool (on the stack).

## Sizing the Heap

The heap is allocated in the space left at the end of the RAM.
See [Ariel OS memory layout][memory-layout-book].

Applications must adjust the value of the laze variable `heapsize_required` with the size (in bytes) that they require:

```yaml
apps:
  - name: example-app
    env:
      global:
        heapsize_required:
          - "32768"
```

`heapsize_required` must be a YAML sequence of strings.
The value given will be added to the variable, *increasing* the size of the heap by the given amount.
The `heapsize_required` variable is initialized to a non-zero value, which might suffice to some applications; it is however strongly recommended to still specify the required heap size explicitly.
Additionally, Ariel OS functionalities that require a larger heap may automatically increase the value of `heapsize_required` as needed.

> [!TIP]
> When the requested heap size does not fit in RAM, the build will fail at link time.

Memory exhaustion results in the application being terminated.
Depending on the allocator and on the item initiating the allocation (e.g., [`Vec::push()`][vec-push-alloc-std-docs]), the application will either panic or abort.

[global-alloc-alloc-std-docs]: https://doc.rust-lang.org/stable/alloc/alloc/trait.GlobalAlloc.html
[laze-modules-book]: ./build-system.md#laze-modules
[no-std-attr-rust]: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
[alloc-std-docs]: https://doc.rust-lang.org/stable/alloc/
[vec-alloc-std-docs]: https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html
[heapless-docsrs]: https://docs.rs/heapless/latest/heapless/
[memory-layout-book]: ./memory-layout.md
[vec-push-alloc-std-docs]: https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html#method.push
