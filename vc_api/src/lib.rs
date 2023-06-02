use ark_bls12_381::Bls12_381;
use ark_bls12_381::{Fq2Parameters, Fq6Parameters, Fr};
use ark_ec::short_weierstrass_jacobian::GroupAffine;
use ark_ec::PairingEngine;
use ark_ff::BigInteger384;
use ark_ff::QuadExtField;
use ark_ff::{BigInteger, BigInteger256};
use ark_ff::{CubicExtField, Fp2ParamsWrapper, Fp6ParamsWrapper};
use ark_std::rand::SeedableRng;
use base64::{decode, encode};
use pointproof::Commitment;
use pointproof::CommitmentScheme;
use pointproof::ProverParam;
use pointproof::StructuredReferenceString;
use pointproof::VerifierParam;
use rand_chacha::ChaCha20Rng;

pub fn gen_params_f1() -> StructuredReferenceString<Bls12_381, 4> {
    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    StructuredReferenceString::<Bls12_381, 4>::new_srs_for_testing(&mut rng)
}

pub fn commit_f1(
    g: Vec<<Bls12_381 as PairingEngine>::G1Affine>,
    h: Vec<<Bls12_381 as PairingEngine>::G2Affine>,
    t: <Bls12_381 as PairingEngine>::Fqk,
    message: Vec<Fr>,
) -> <Bls12_381 as PairingEngine>::G1Projective {
    let param = StructuredReferenceString { g, h, t };
    let prover_param: ProverParam<Bls12_381, 4> = (&param).into();
    Commitment::<Bls12_381, 4>::commit(&prover_param, &message).commitment
}

pub fn open_f1(
    g: Vec<<Bls12_381 as PairingEngine>::G1Affine>,
    h: Vec<<Bls12_381 as PairingEngine>::G2Affine>,
    t: <Bls12_381 as PairingEngine>::Fqk,
    message: Vec<Fr>,
    pos: usize,
) -> <Bls12_381 as PairingEngine>::G1Projective {
    let param = StructuredReferenceString { g, h, t };
    let prover_param: ProverParam<Bls12_381, 4> = (&param).into();
    Commitment::<Bls12_381, 4>::open(&prover_param, &message, pos)
}

pub fn verify_f1(
    g: Vec<<Bls12_381 as PairingEngine>::G1Affine>,
    h: Vec<<Bls12_381 as PairingEngine>::G2Affine>,
    t: <Bls12_381 as PairingEngine>::Fqk,
    commitment: <Bls12_381 as PairingEngine>::G1Projective,
    message: Fr,
    pos: usize,
    witness: <Bls12_381 as PairingEngine>::G1Projective,
) -> bool {
    let param = StructuredReferenceString { g, h, t };
    let commitment = Commitment { commitment };
    let verifier_param: VerifierParam<Bls12_381, 4> = (&param).into();
    commitment.verify(&verifier_param, &message, pos, &witness)
}

fn convert_bits_to_bytes(mut bits: Vec<bool>) -> Vec<u8> {
    while bits.len() % 8 != 0 {
        bits.push(false);
    }
    let mut res = vec![];
    for i in (0..bits.len()).step_by(8) {
        let mut x = 0;
        for j in (0..8).rev() {
            x = (x << 1) | (bits[i + j] as u8);
        }
        res.push(x);
    }
    res
}

fn convert_bytes_to_bits(bytes: &Vec<u8>) -> Vec<bool> {
    let mut res = vec![];
    for i in 0..bytes.len() {
        let mut x = bytes[i];
        for _j in 0..8 {
            res.push(x % 2 == 1);
            x >>= 1;
        }
    }
    res
}

fn bytes_to_g1(bytes: [Vec<u8>; 3]) -> <Bls12_381 as PairingEngine>::G1Affine {
    let x = BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[0]));
    let y = BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[1]));
    let infinity = convert_bytes_to_bits(&bytes[2])[0];
    GroupAffine::new(x.into(), y.into(), infinity)
}

fn g1_to_bytes(g1: <Bls12_381 as PairingEngine>::G1Affine) -> [Vec<u8>; 3] {
    let int_x: BigInteger384 = g1.x.into();
    let x = convert_bits_to_bytes(int_x.to_bits_le());
    let int_y: BigInteger384 = g1.y.into();
    let y = convert_bits_to_bytes(int_y.to_bits_le());
    let infinity = convert_bits_to_bytes(vec![g1.infinity]);
    [x, y, infinity]
}

fn bytes_to_g2(bytes: [Vec<u8>; 5]) -> <Bls12_381 as PairingEngine>::G2Affine {
    GroupAffine::new(
        QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
            BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[0])).into(),
            BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[1])).into(),
        ),
        QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
            BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[2])).into(),
            BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[3])).into(),
        ),
        convert_bytes_to_bits(&bytes[4])[0],
    )
}

fn g2_to_bytes(g2: <Bls12_381 as PairingEngine>::G2Affine) -> [Vec<u8>; 5] {
    let mut res = vec![];
    let quad_ext_field = g2.x;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let quad_ext_field = g2.y;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let infinity = convert_bits_to_bytes(vec![g2.infinity]);
    res.push(infinity);

    res.try_into().unwrap()
}

fn bytes_to_quad_ext_field(bytes: [Vec<u8>; 12]) -> <Bls12_381 as PairingEngine>::Fqk {
    <Bls12_381 as PairingEngine>::Fqk::new(
        CubicExtField::<Fp6ParamsWrapper<Fq6Parameters>>::new(
            QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[0])).into(),
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[1])).into(),
            ),
            QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[2])).into(),
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[3])).into(),
            ),
            QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[4])).into(),
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[5])).into(),
            ),
        ),
        CubicExtField::<Fp6ParamsWrapper<Fq6Parameters>>::new(
            QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[6])).into(),
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[7])).into(),
            ),
            QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[8])).into(),
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[9])).into(),
            ),
            QuadExtField::<Fp2ParamsWrapper<Fq2Parameters>>::new(
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[10])).into(),
                BigInteger384::from_bits_le(&convert_bytes_to_bits(&bytes[11])).into(),
            ),
        ),
    )
}

fn quad_ext_field_to_bytes(t: <Bls12_381 as PairingEngine>::Fqk) -> [Vec<u8>; 12] {
    let mut res = vec![];
    let cubic_ext_field = t.c0;

    let quad_ext_field = cubic_ext_field.c0;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let quad_ext_field = cubic_ext_field.c1;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let quad_ext_field = cubic_ext_field.c2;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let cubic_ext_field = t.c1;

    let quad_ext_field = cubic_ext_field.c0;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let quad_ext_field = cubic_ext_field.c1;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    let quad_ext_field = cubic_ext_field.c2;
    let i: BigInteger384 = quad_ext_field.c0.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));
    let i: BigInteger384 = quad_ext_field.c1.into();
    res.push(convert_bits_to_bytes(i.to_bits_le()));

    res.try_into().unwrap()
}

fn str_to_srs(s: &str) -> StructuredReferenceString<Bls12_381, 4> {
    let strings = s.split_ascii_whitespace().collect::<Vec<&str>>();
    assert!(strings.len() == 3);
    let g_str = strings[0]
        .split(";")
        .filter(|&x| x.len() > 0)
        .collect::<Vec<&str>>();
    let mut g = vec![];
    for i in g_str {
        let bytes = i
            .split(",")
            .filter(|&x| x.len() > 0)
            .map(|x| decode(x).unwrap())
            .collect::<Vec<Vec<u8>>>()
            .try_into()
            .unwrap();
        g.push(bytes_to_g1(bytes));
    }

    let h_str = strings[1]
        .split(";")
        .filter(|&x| x.len() > 0)
        .collect::<Vec<&str>>();
    let mut h = vec![];
    for i in h_str {
        let bytes = i
            .split(",")
            .filter(|&x| x.len() > 0)
            .map(|x| decode(x).unwrap())
            .collect::<Vec<Vec<u8>>>()
            .try_into()
            .unwrap();
        h.push(bytes_to_g2(bytes));
    }
    let t_bytes = strings[2]
        .split(",")
        .filter(|&x| x.len() > 0)
        .map(|x| decode(x).unwrap())
        .collect::<Vec<Vec<u8>>>()
        .try_into()
        .unwrap();
    StructuredReferenceString {
        g,
        h,
        t: bytes_to_quad_ext_field(t_bytes),
    }
}

fn srs_to_str(srs: StructuredReferenceString<Bls12_381, 4>) -> String {
    let mut res = String::new();
    for i in srs.g {
        let bytes = g1_to_bytes(i);
        for j in bytes {
            res.push_str(encode(j).as_str());
            res.push(',');
        }
        res.push(';');
    }
    res.push('\n');
    for i in srs.h {
        let bytes = g2_to_bytes(i);
        for j in bytes {
            res.push_str(encode(j).as_str());
            res.push(',');
        }
        res.push(';');
    }
    res.push('\n');
    for j in quad_ext_field_to_bytes(srs.t) {
        res.push_str(encode(j).as_str());
        res.push(',');
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srs_str() {
        let srs = gen_params_f1();
        let s = srs_to_str(srs.clone());
        let srs1 = str_to_srs(s.as_str());
        assert_eq!(srs, srs1);
    }

    #[test]
    fn it_works() {
        // let srs = gen_params_f1();
        // let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        // let v: Vec<ark_ff::Fp256<FrParameters>> = (0..4).map(|_| <Bls12_381 as PairingEngine>::Fr::rand(&mut rng)).collect();
        // let commitment = commit_f1(srs.0.clone(), srs.1.clone(), srs.2, v.clone());
        // let witness = open_f1(srs.0.clone(), srs.1.clone(), srs.2, v.clone(), 2);
        // assert!(verify_f1(srs.0, srs.1, srs.2, commitment, v[2], 2, witness))
    }
}
