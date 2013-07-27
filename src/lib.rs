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
trait Arbitrary : Clone {
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
 * quick_check
 *   description: a string describing what the property being tested is
 *   f: function that takes arbitrary input and returns whether
 *      the invariant held.
 *
 *   returns whether all the tests passed.
 */
fn quick_check<T:Arbitrary>(description : &str,
                            f : extern fn(T) -> bool) -> bool {
    let num_tests = 100;
    let mut passing = 0;
    let mut failed = ~[];
    for num_tests.times {
        let t : T = Arbitrary::gen();
        if f(t.clone()) {
            passing += 1;
        } else {
            failed.push(t);
        }
    }
    if passing == num_tests {
        println(fmt!("+++ OK, passed 100 tests for %s", description));
        return true;
    } else {
        println(fmt!("*** Failed '%s' on:", description));
        if failed.len() < 5 {
            // is this really the best way? can't be...
            let mut i = 0;
            for failed.len().times {
                println(fmt!("%?", failed[i]));
                i += 1;
            }
        } else {
            let mut failed_with_sizes : ~[(uint, T)] =
                failed.iter().transform(|x| (x.size(), x.clone()))
                .collect();
            sort::quick_sort(failed_with_sizes, |e1, e2| {
                e1.first() <= e2.first()
            });
            let mut i = 0;
            for 5.times {
                println(fmt!("%?", failed_with_sizes[i].second()));
                i += 1;
            }
            println("...and more");
        }
        return false;
    }
}

/**
 * quick_check2
 *   description: a string describing what the property being tested is
 *   f: function that takes two arbitrary inputs and returns whether
 *      the invariant held.
 *
 *   returns whether all the tests passed.
 */
fn quick_check2<T:Arbitrary>(description : &str,
                             f : extern fn(T,T) -> bool) -> bool {
    let num_tests = 100;
    let mut passing = 0;
    let mut failed = ~[];
    for num_tests.times {
        let t1 : T = Arbitrary::gen();
        let t2 : T = Arbitrary::gen();
        if f(t1.clone(), t2.clone()) {
            passing += 1;
        } else {
            failed.push((t1, t2));
        }
    }
    if passing == num_tests {
        println(fmt!("+++ OK, passed 100 tests for %s", description));
        return true;
    } else {
        println(fmt!("*** Failed '%s' on:", description));
        if failed.len() < 5 {
            // is this really the best way? can't be...
            let mut i = 0;
            for failed.len().times {
                println(fmt!("%?", failed[i]));
                i += 1;
            }
        } else {
            let mut failed_with_sizes : ~[(uint, (T,T))] =
                failed.iter().transform(|x| (x.first().size() + x.second().size(),
                                             x.clone()))
                .collect();
            sort::quick_sort(failed_with_sizes, |e1, e2| {
                e1.first() <= e2.first()
            });
            let mut i = 0;
            for 5.times {
                println(fmt!("%?", failed_with_sizes[i].second()));
                i += 1;
            }
            println("...and more");
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
        quick_check("reversing a list twice yields the same list",
                    prop_reverse_reverse_uints));
    assert!(
        !quick_check("reversing a list moves first to last",
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
        !quick_check2("add_foos is commutative",
                    prop_add_foos_commutes));

    assert!(
        quick_check("add_foos with a zero Foo is an identity",
                    prop_add_zero_foo_identity));
}