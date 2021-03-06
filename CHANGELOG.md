# CHANGELOG

## v0.0.6
* Removes `proc-macro-hack` dependency.
  * Starting on rust 1.45, function-like proc macros are allowed in
    statement expressions
* Allow autoderiving `Clone`.
  * It will panic when trying to clone a mocked instance but work as
    expected on real instances
  * [test](/tests/clone.rs)
* Added `then_do`
  * It explicitly avoids letting the user look at the input parameters
    such that it can be safe to use on methods with non-static
    arguments
* Added `then_return`
  * Allows users to easily and safely mock the return value of methods
    without using a closure

### Breaking Change
* Minimum rust version changed to 1.45

## v0.0.5
* Suppport `impl` arguments in mocked methods
  * [test](/tests/generic_methods.rs)

## v0.0.4
* Mocks can now be called an infinite number of times
  * [test](/tests/multi_mock.rs)
* Trait implementations may now be mocked
  * [tests](/tests/trait_impl.rs)
  * This is a very MVP release for the feature. A limitation that may
    be lifted in the future is that you may not have two methods with
    the same name, even if one is through a trait implementation.
* Better macro error messages
  * [PR](https://github.com/nrxus/faux/pull/23). Thanks a lot [@TedDriggs](https://github.com/TedDriggs)!
* `self: Box<Self>`, `self: Rc<Self>`, and `self: Arc<Self>` are now
  allowed in methods inside an impl blocked tagged by
  `#[methods]`. See the tests for what is possible with arbitrary self
  types.
  * [tests](/tests/arbitrary_self.rs)

### Breaking Change

* Specifying a path in `#[methods]` has changed from:
  `#[methods(path::to::mod)]` to `#[methods(path =
  "path::to::mod")]`. This is done to allow further current and future
  arguments to be passed to the attribute.
* The default behavior for mocks has changed to be active for multiple
  calls. As a result, now mocks only accept `FnMut` closures which
  cannot have environment data moved into. If your mocks used to
  depend on moving data into the closure for the mock, you can use the
  `once` method on `When` to go back to the old behavior.

## v0.0.3
*  Async methods may now be mocked
   * [tests](/tests/asynchronous.rs)
* Mocked structs may now contain generics
   * [tests](/tests/generic_struct.rs)
* Mocked impl blocks may now contain a path
   * Paths that contain `super` or `crate` have to use the
     `faux::methods(path)` syntax.
   * [tests](/tests/paths.rs)

## v0.0.2

* Made mocked structs respect Sync+Send+Debug.
   * This allows mocking structs that are enforced to be
     Sync/Send/Debug. This has a performance impact as we now used
     Mutex rather than RefCell for interor mutability. This is unideal
     and will be addressed in a future release.

### Breaking Change

* All closures used for mocking methods must now implement Send. This
  means that all of its captures variables need to implement
  Send. This is unideal but needed for now to be able to mock things
  like Rocket's request guards.

## v0.0.1

* Initial alpha release
