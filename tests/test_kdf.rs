use hpke::kdf;

mod test_util;
use test_util::*;

#[test]
fn test_hkdf_sha256() {
    let kdf = kdf::Kdf::new(kdf::Mode::HkdfSha256);

    let ikm = hex_to_bytes("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
    let salt = hex_to_bytes("000102030405060708090a0b0c");
    let info = hex_to_bytes("f0f1f2f3f4f5f6f7f8f9");
    let len = 42;

    let expected_prk =
        hex_to_bytes("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5");
    let expected_okm = hex_to_bytes(
        "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865",
    );

    let prk = kdf.extract(&salt, &ikm);
    let okm = kdf.expand(&prk, &info, len);

    assert_eq!(&expected_prk, &prk);
    assert_eq!(&expected_okm, &okm);
}
