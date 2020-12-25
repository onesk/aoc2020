const DH1: usize = 1614360;
const DH2: usize = 7734663;

const GEN: usize = 7;
const MODULO: usize = 20201227;

fn dlog(exp: usize) -> usize {
    let mut cur = 1usize;

    for dlog in 0.. {
        if cur == exp {
            println!("dlog {}", dlog);
            return dlog;
        }

        cur *= GEN;
        cur %= MODULO;
    }

    unreachable!()
}

fn exp(dlog: usize) -> usize {
    let mut ret = 1usize;
    let mut pow2 = dlog.next_power_of_two();

    while pow2 > 0 {
        ret = (ret * ret) % MODULO;

        if pow2 & dlog != 0 {
            ret = (ret * GEN) % MODULO;
        }

        pow2 >>= 1;
    }

    ret
}

fn part_one(dh1: usize, dh2: usize) -> usize {
    exp(dlog(dh1) * dlog(dh2))
}

fn main() {
    println!("{}", part_one(DH1, DH2));
}

#[test]
fn example() {
    assert_eq!(part_one(5764801, 17807724), 14897079);
}
