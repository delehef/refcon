use rand::Rng;

use crate::Refcon;

#[derive(Clone)]
struct AFatStruct {
    _a: f32,
    _b: usize,
    _c: [u8; 16],
}
impl AFatStruct {
    fn make_some() -> AFatStruct {
        let mut rng = rand::thread_rng();
        AFatStruct {
            _a: rng.gen(),
            _b: rng.gen(),
            _c: rng.gen(),
        }
    }
}

const SOME_ZERO: AFatStruct = AFatStruct {
    _a: 0.32,
    _b: 12,
    _c: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
};

#[test]
fn mixed_vec() {
    #[inline(never)]
    fn make_refcon_vec(count: usize) -> Vec<Refcon<'static, AFatStruct>> {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            let x = if i % 10 == 0 {
                Refcon::from(&SOME_ZERO)
            } else {
                Refcon::from(AFatStruct::make_some())
            };
            v.push(x)
        }
        v
    }

    #[inline(never)]
    fn make_copied_vec(count: usize) -> Vec<AFatStruct> {
        let mut v = Vec::with_capacity(count);
        for i in 0..count {
            let x = if i % 10 == 0 {
                SOME_ZERO.clone()
            } else {
                AFatStruct::make_some()
            };
            v.push(x)
        }
        v
    }

    let options = microbench::Options::default();
    microbench::bench(&options, "copy", || make_copied_vec(1_000_000));
    microbench::bench(&options, "refcon", || make_refcon_vec(1_000_000));
}
