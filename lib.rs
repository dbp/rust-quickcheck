#[ link(name = "quickcheck",
        vers = "0.1",
        uuid = "510d3dae-f7b1-11e2-83f1-bb59638dec3b") ];

#[ desc = "Property based randomized testing library" ];
#[ license = "ISC" ];
#[ author = "Daniel Patterson" ];

#[ crate_type = "lib" ];

#[ warn(non_camel_case_types) ];

extern mod extra;
use std::rand;
use std::vec;
use std::num;
use extra::sort;

/**
 * Arbitrary is a trait for types that you can generate arbitrary
 * instances of. You should also be able to ask for their size, so
 * you can report small failing examples instead of large ones.
 *
 */
pub trait Arbitrary : Clone {
    fn gen() -> Self;
    fn size(&self) -> uint;
}

impl Arbitrary for uint {
    fn gen() -> uint {
        let n : uint = rand::random();
         n % num::pow_with_uint(2,10)
    }
    fn size(&self) -> uint {
        self.clone()
    }
}

impl Arbitrary for int {
    fn gen() -> int {
        let n : int = rand::random();
        n % num::pow_with_uint(2,10)
    }

    fn size(&self) -> uint {
        self.clone().to_uint()
    }
}

impl<T:Arbitrary> Arbitrary for ~[T] {
    fn gen() -> ~[T] {
        let _n : uint = rand::random();
        let n = _n % 100;
        vec::build_sized(n, |p| {
            for n.times {
                p(Arbitrary::gen());
            }
        })
    }

    fn size(&self) -> uint {
        self.len()
    }
}

/**
 * Result is what is returned from property tests; it includes
 * representations of the generated input in the case of failure.
 */
#[deriving(Clone)]
enum Result {
    Pass,
    Failure(uint, ~str)
}

fn result_str(r : ~Result) -> ~str {
    match r {
        ~Pass => ~"",
        ~Failure(_, s) => s.clone()
    }
}

/**
 * Testable is a trait for things that can be tested. This generally means
 * functions that take arguments of trait Arbitrary.
 */
pub trait Testable {
    fn apply(&self) -> ~Result;
}

impl<T:Arbitrary> Testable for extern fn(T) -> bool {
    fn apply(&self) -> ~Result {
        let t : T = Arbitrary::gen();
        if (*self)(t.clone()) {
            ~Pass
        } else {
            ~Failure(t.size(), fmt!("%?",t))
        }
    }
}


impl<T:Arbitrary, U:Arbitrary> Testable for extern fn(T,U) -> bool {
    fn apply(&self) -> ~Result {
        let t : T = Arbitrary::gen();
        let u : U = Arbitrary::gen();
        if (*self)(t.clone(), u.clone()) {
            ~Pass
        } else {
            ~Failure(t.size() + u.size(), fmt!("%?, %?",t, u))
        }
    }
}

/**
 * quick_check
 *   description: a string describing what the property being tested is
 *   f: a Testable (ie, function taking arguments that are Arbitrary)
 *
 *   returns whether all the tests passed.
 */
pub fn quick_check<F:Testable>(description: &str, f: F) -> bool {
    quick_check_internal(description, f, false)
}

/**
 * quick_check_silent
 *   description: a string describing what the property being tested is
 *   f: a Testable (ie, function taking arguments that are Arbitrary)
 *
 *   returns whether all the tests passed.
 *
 * note: does not print out any output.
 */
pub fn quick_check_silent<F:Testable>(description: &str, f: F) -> bool {
    quick_check_internal(description, f, true)
}


fn quick_check_internal<F:Testable>(description: &str,
                                    f: F,
                                    silent: bool) -> bool {
    let num_tests = 100;
    let mut passing = 0;
    let mut failed = ~[];
    for num_tests.times {
        match f.apply() {
            ~Pass            => { passing += 1; }
            ~Failure(s,args) => { failed.push(~Failure(s,args)); }
        }
    }
    if passing == num_tests {
        if !silent {
            println(fmt!("+++ OK, passed 100 tests for %s", description));
        }
        return true;
    } else {
        if !silent {
            println(fmt!("*** Failed '%s' on:", description));
            if failed.len() < 5 {
                // is this really the best way? can't be...
                let mut i = 0;
                for failed.len().times {
                    println(fmt!("%?", failed[i].clone()));
                    i += 1;
                }
            } else {
                sort::quick_sort(failed, |e1, e2| {
                    match (e1,e2) {
                        (&~Failure(s1,_), &~Failure(s2,_)) => { s1 <= s2 }
                        _ => { fail!() }
                    }
                });
                let mut i = 0;
                for 5.times {
                    println(fmt!("%?", result_str(failed[i].clone())));
                    i += 1;
                }
                println("...and more");
            }
        }
        return false;
    }
}

#[test]
fn reverse_uint_vecs() {
    fn reverse<A:Clone>(v : ~[A]) -> ~[A] {
        let mut newvec = ~[];
        for v.iter().advance |e| {
            // NOTE(dbp 2013-07-25): This is intentionally buggy - should be unshift.
            newvec.push(e.clone());
        }
        return newvec;
    }

    fn prop_reverse_reverse_uints(v : ~[uint]) -> bool {
        reverse(reverse(v.clone())) == v
    }

    fn prop_reverse_moves_first_to_last(v : ~[uint]) -> bool {
        if v.len() > 0 {
            reverse(v.clone())[v.len() - 1] == v[0]
        } else {
            true // trivially
        }
    }

    assert!(
        quick_check_silent("reversing a list twice yields the same list",
                           prop_reverse_reverse_uints));
    assert!(
        !quick_check_silent("reversing a list moves first to last",
                       prop_reverse_moves_first_to_last));
}

#[test]
fn struct_gen() {

    #[deriving(Clone, Eq)]
    struct Foo { n: uint, xs: ~[int] }

    impl Arbitrary for Foo {
        fn gen() -> Foo {
            Foo { n: Arbitrary::gen(), xs: Arbitrary::gen() }
        }
        fn size(&self) -> uint {
            self.xs.len()
        }
    }
    // NOTE(dbp 2013-07-27): Not sure why this isn't covered by `deriving`,
    // is it a not-yet-implemented feature?
    impl Clone for Foo {
        fn clone(&self) -> Foo {
            Foo { n: self.n, xs: self.xs.clone() }
        }
    }
    impl Eq for Foo {
        fn eq(&self, other: &Foo) -> bool {
            self.n == other.n && self.xs == other.xs
        }
    }

    fn add_foos(a1: &Foo, a2: &Foo) -> Foo {
        let mut new_xs = ~[];
        new_xs = vec::append(new_xs, a1.xs);
        new_xs = vec::append(new_xs, a2.xs);
        Foo {n: a1.n + a2.n, xs: new_xs}
    }

    // NOTE(dbp 2013-07-27): This is obviously not true.
    fn prop_add_foos_commutes(a1: Foo, a2: Foo) -> bool {
        add_foos(&a1, &a2) == add_foos(&a2, &a1)
    }

    fn prop_add_zero_foo_identity(a: Foo) -> bool {
        add_foos(&a, &Foo {n: 0, xs: ~[]}) == a
    }

    assert!(
        !quick_check_silent("add_foos is commutative",
                            prop_add_foos_commutes));

    assert!(
        quick_check_silent("add_foos with a zero Foo is an identity",
                           prop_add_zero_foo_identity));
}