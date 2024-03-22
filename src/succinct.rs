use crate::{arith::U256, groups, AffineG1, FieldError, Fq, Fr, Group, GroupError, G1};

extern "C" {
    pub fn syscall_bn254_add(p: *mut u32, q: *const u32);
    pub fn syscall_bn254_double(p: *mut u32);
}


#[inline]
pub fn point_to_le(p: &G1) -> Result<[u8; 64], FieldError> {
    let mut b = [0u8; 64];
    p.x().to_big_endian(&mut b[32..])?;
    p.y().to_big_endian(&mut b[..32])?;
    b.reverse();
    Ok(b)
}

#[inline]
pub fn le_to_point(b: &[u8]) -> Result<G1, FieldError> {
    let mut bx = b[0..32].to_vec();
    bx.reverse();
    let mut by = b[32..64].to_vec();
    by.reverse();
    let px = read_fq(&bx)?;
    let py = read_fq(&by)?;
    let p = new_g1_point(px, py).expect("G1 point is not in group");
    Ok(p)
}


#[inline]
pub fn read_fq(b: &[u8]) -> Result<Fq, FieldError> {
    Fq::from_slice(&b[..32])
}

pub fn new_g1_point(px: Fq, py: Fq) -> Result<G1, GroupError> {
    if px == Fq::zero() && py == Fq::zero() {
        Ok(G1::zero())
    } else {
        AffineG1::new(px, py)
            .map(Into::into)
    }
}


#[test]
fn test_le_to_point() {
    let b: [u8; 64] = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let p = le_to_point(&b).unwrap();
    assert_eq!(p, G1::one());
}

#[test]
fn test_point_to_le() {
    let p = G1::one();
    let b = point_to_le(&p).unwrap();
    assert_eq!(
        b,
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ]
    );
}