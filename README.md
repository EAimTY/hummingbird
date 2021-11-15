# no_name_yet

WIP

no_name_yet is a lightweight CMS uses Git as the database.

no_name_yet is very much still work in progress. Do not use it in a production environment.

## About the unsafe impl Send for git2::Repository

`no_name_yet` uses [git2-rs](https://github.com/rust-lang/git2-rs) to operate git repositories.
Structs in `git2-rs` are `!Send` as `git2-rs` is a *safe* Rust bind using raw pointers to interact with the C library `libgit2`.

The official doc about threading for `libgit2`: https://github.com/libgit2/libgit2/blob/main/docs/threading.md

As mentioned in the doc, `libgit2`'s objects cannot be safely accessed by multiple threads simultaneously. But in `no_name_yet`, the repository can only be updated by calling the `update()` method of `Database`, which has already being protected with an `RwLock`. No parallel operation will happens on `git2::Repository`

Still, This is just a rough speculation that `Repository` can be safely passed between threads, so the `send-repo` branch was created for verifying the theory above.
