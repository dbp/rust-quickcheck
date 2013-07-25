extern mod extra;
use std::rand;
use std::vec;
use std::num;
use extra::sort;

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

fn check<T:Arbitrary>(description : &str, f : extern fn(T) -> bool) {
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
        println("+++ OK, passed 100 tests");
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
                println(fmt!("%?", failed[i]));
                i += 1;
            }
            println("...and more");
        }
    }
}

fn main() {
    fn reverse<A:Clone>(v : ~[A]) -> ~[A] {
        // fancy optimization!
        if (v.len() == 2) {
            v
        } else {
            let mut newvec = ~[];
            for v.iter().advance |e| {
                newvec.unshift(e.clone());
            }
            return newvec;
        }
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

    check("reversing a list twice yields the same list",
          prop_reverse_reverse_uints);
    check("reversing a list moves first element to last",
          prop_reverse_moves_first_to_last);
}