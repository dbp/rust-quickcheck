## About

This is an implementation of QuickCheck, in **very very** early
stages.

## Example

Here's an example use:

    extern mod quickcheck;

    use quickcheck::*;

    fn main() {
        fn foo(i: uint) -> bool {
            true
        }

        quick_check("a useless test", foo);
    }


## Prior work

There was a previous attempt at this at [1], but it doesn't really
have anything to build upon (it is _very_ incomplete). There are also
notes about it at [2], but no actual work, as far as I can tell. There is an issue at [3].

## Design

Following the original Haskell design, the main idea is to have a
trait that allows you to generate arbitrary instances of values. Then
you write properties over those values, and the library generates
values for you and runs the properties for you.


1. https://github.com/mcandre/rustcheck/blob/master/rustcheck.rs
2. https://gist.github.com/jruderman/4617375
3. https://github.com/mozilla/rust/issues/7232