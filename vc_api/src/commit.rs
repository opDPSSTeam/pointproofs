use ark_bls12_381::Bls12_381;
use ark_bls12_381::Fr;
use ark_ec::PairingEngine;
use pointproof::Commitment;
use pointproof::CommitmentScheme;
use pointproof::ProverParam;
use pointproof::StructuredReferenceString;
use pointproof::VerifierParam;
use rand_chacha::ChaCha20Rng;

pub fn gen_params<const N: usize>(
    rng: &mut ChaCha20Rng
) -> StructuredReferenceString<Bls12_381, N> {
    StructuredReferenceString::<Bls12_381, N>::new_srs_for_testing(rng)
}

pub fn commit<const N: usize>(
    srs: StructuredReferenceString<Bls12_381, N>,
    message: Vec<Fr>,
) -> <Bls12_381 as PairingEngine>::G1Affine {
    let prover_param: ProverParam<Bls12_381, N> = (&srs).into();
    Commitment::<Bls12_381, N>::commit(&prover_param, &message)
        .commitment
        .into()
}

pub fn open<const N: usize>(
    srs: StructuredReferenceString<Bls12_381, N>,
    message: Vec<Fr>,
    pos: usize,
) -> <Bls12_381 as PairingEngine>::G1Affine {
    let prover_param: ProverParam<Bls12_381, N> = (&srs).into();
    Commitment::<Bls12_381, N>::open(&prover_param, &message, pos).into()
}

pub fn verify<const N: usize>(
    srs: StructuredReferenceString<Bls12_381, N>,
    commitment: <Bls12_381 as PairingEngine>::G1Affine,
    message: Fr,
    pos: usize,
    witness: <Bls12_381 as PairingEngine>::G1Affine,
) -> bool {
    let commitment = Commitment {
        commitment: commitment.into(),
    };
    let verifier_param: VerifierParam<Bls12_381, N> = (&srs).into();
    commitment.verify(&verifier_param, &message, pos, &witness.into())
}