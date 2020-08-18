// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use super::psa_crypto_binding::{
    self, psa_algorithm_t, psa_dh_group_t, psa_ecc_curve_t, psa_key_attributes_t,
    psa_key_derivation_operation_t, psa_key_id_t, psa_key_lifetime_t, psa_key_type_t,
    psa_key_usage_t,
};

pub unsafe fn psa_get_key_bits(attributes: *const psa_key_attributes_t) -> usize {
    psa_crypto_binding::shim_get_key_bits(attributes)
}

pub unsafe fn psa_get_key_type(attributes: *const psa_key_attributes_t) -> psa_key_type_t {
    psa_crypto_binding::shim_get_key_type(attributes)
}

pub unsafe fn psa_get_key_lifetime(attributes: *const psa_key_attributes_t) -> psa_key_lifetime_t {
    psa_crypto_binding::shim_get_key_lifetime(attributes)
}

pub unsafe fn psa_get_key_algorithm(attributes: *const psa_key_attributes_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_get_key_algorithm(attributes)
}

pub unsafe fn psa_get_key_usage_flags(attributes: *const psa_key_attributes_t) -> psa_key_usage_t {
    psa_crypto_binding::shim_get_key_usage_flags(attributes)
}

pub unsafe fn psa_key_attributes_init() -> psa_key_attributes_t {
    psa_crypto_binding::shim_key_attributes_init()
}

pub fn psa_key_derivation_operation_init() -> psa_key_derivation_operation_t {
    unsafe { psa_crypto_binding::shim_key_derivation_operation_init() }
}

pub unsafe fn psa_set_key_algorithm(attributes: *mut psa_key_attributes_t, alg: psa_algorithm_t) {
    psa_crypto_binding::shim_set_key_algorithm(attributes, alg);
}

pub unsafe fn psa_set_key_bits(attributes: *mut psa_key_attributes_t, bits: usize) {
    psa_crypto_binding::shim_set_key_bits(attributes, bits);
}

pub unsafe fn psa_set_key_id(attributes: *mut psa_key_attributes_t, id: psa_key_id_t) {
    psa_crypto_binding::shim_set_key_id(attributes, id);
}

pub unsafe fn psa_set_key_lifetime(
    attributes: *mut psa_key_attributes_t,
    lifetime: psa_key_lifetime_t,
) {
    psa_crypto_binding::shim_set_key_lifetime(attributes, lifetime);
}

pub unsafe fn psa_set_key_type(attributes: *mut psa_key_attributes_t, type_: psa_key_type_t) {
    psa_crypto_binding::shim_set_key_type(attributes, type_);
}

pub unsafe fn psa_set_key_usage_flags(
    attributes: *mut psa_key_attributes_t,
    usage_flags: psa_key_usage_t,
) {
    psa_crypto_binding::shim_set_key_usage_flags(attributes, usage_flags);
}

pub unsafe fn psa_get_key_id(attributes: *const psa_key_attributes_t) -> psa_key_id_t {
    psa_crypto_binding::shim_get_key_id(attributes)
}

pub fn PSA_ALG_IS_HASH(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_HASH(alg) == 1 }
}

pub fn PSA_ALG_IS_MAC(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_MAC(alg) == 1 }
}

pub fn PSA_ALG_IS_HMAC(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_HMAC(alg) == 1 }
}

pub fn PSA_ALG_IS_BLOCK_CIPHER_MAC(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_BLOCK_CIPHER_MAC(alg) == 1 }
}

pub fn PSA_ALG_IS_FULL_LENGTH_MAC(alg: psa_algorithm_t) -> bool {
    // Not in PSA spec but required to convert from psa_alg_t to algorithm/Mac
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_FULL_LENGTH_MAC(alg) == 1 }
}

pub fn PSA_ALG_IS_CIPHER(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_CIPHER(alg) == 1 }
}

pub fn PSA_ALG_IS_AEAD(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_AEAD(alg) == 1 }
}

pub fn PSA_ALG_IS_SIGN(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_SIGN(alg) == 1 }
}

pub fn PSA_ALG_IS_ASYMMETRIC_ENCRYPTION(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_ASYMMETRIC_ENCRYPTION(alg) == 1 }
}

pub unsafe fn PSA_ALG_IS_RSA_OAEP(alg: psa_algorithm_t) -> bool {
    psa_crypto_binding::shim_PSA_ALG_IS_RSA_OAEP(alg) == 1
}

pub fn PSA_ALG_IS_KEY_AGREEMENT(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_KEY_AGREEMENT(alg) == 1 }
}

pub fn PSA_ALG_IS_RAW_KEY_AGREEMENT(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_RAW_KEY_AGREEMENT(alg) == 1 }
}

pub fn PSA_ALG_IS_FFDH(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_FFDH(alg) == 1 }
}

pub fn PSA_ALG_IS_ECDH(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_FFDH(alg) == 1 }
}

pub fn PSA_ALG_IS_KEY_DERIVATION(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_KEY_DERIVATION(alg) == 1 }
}

pub fn PSA_ALG_IS_RSA_PKCS1V15_SIGN(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_RSA_PKCS1V15_SIGN(alg) == 1 }
}

pub fn PSA_ALG_IS_RSA_PSS(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_RSA_PSS(alg) == 1 }
}

pub fn PSA_ALG_IS_ECDSA(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_ECDSA(alg) == 1 }
}

pub fn PSA_ALG_IS_DETERMINISTIC_ECDSA(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_DETERMINISTIC_ECDSA(alg) == 1 }
}

pub fn PSA_ALG_IS_HKDF(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_HKDF(alg) == 1 }
}

pub fn PSA_ALG_IS_TLS12_PRF(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_TLS12_PRF(alg) == 1 }
}

pub fn PSA_ALG_IS_TLS12_PSK_TO_MS(alg: psa_algorithm_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_ALG_IS_TLS12_PSK_TO_MS(alg) == 1 }
}

pub fn PSA_ALG_SIGN_GET_HASH(sign_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_SIGN_GET_HASH(sign_alg) }
}

pub fn PSA_ALG_RSA_OAEP_GET_HASH(rsa_oaep_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_RSA_OAEP_GET_HASH(rsa_oaep_alg) }
}

pub fn PSA_ALG_HMAC_GET_HASH(hmac_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_HMAC_GET_HASH(hmac_alg) }
}

pub fn PSA_ALG_HKDF_GET_HASH(hkdf_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_HKDF_GET_HASH(hkdf_alg) }
}

pub fn PSA_ALG_TLS12_PRF_GET_HASH(tls12_prf_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_TLS12_PRF_GET_HASH(tls12_prf_alg) }
}

pub fn PSA_ALG_TLS12_PSK_TO_MS_GET_HASH(tls12_psk_to_ms_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_TLS12_PSK_TO_MS_GET_HASH(tls12_psk_to_ms_alg) }
}

pub unsafe fn PSA_ALG_KEY_AGREEMENT_GET_BASE(alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_KEY_AGREEMENT_GET_BASE(alg)
}

pub unsafe fn PSA_ALG_KEY_AGREEMENT_GET_KDF(alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_KEY_AGREEMENT_GET_KDF(alg)
}

pub fn PSA_ALG_RSA_PKCS1V15_SIGN(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_RSA_PKCS1V15_SIGN(hash_alg) }
}

pub fn PSA_ALG_RSA_PSS(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_RSA_PSS(hash_alg) }
}

pub fn PSA_ALG_ECDSA(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_ECDSA(hash_alg) }
}

pub fn PSA_ALG_DETERMINISTIC_ECDSA(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    unsafe { psa_crypto_binding::shim_PSA_ALG_DETERMINISTIC_ECDSA(hash_alg) }
}

pub unsafe fn PSA_ALG_RSA_OAEP(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_RSA_OAEP(hash_alg)
}

pub unsafe fn PSA_ALG_HMAC(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_HMAC(hash_alg)
}

pub unsafe fn PSA_ALG_TRUNCATED_MAC(
    mac_alg: psa_algorithm_t,
    mac_length: usize,
) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_TRUNCATED_MAC(mac_alg, mac_length)
}

pub unsafe fn PSA_ALG_FULL_LENGTH_MAC(mac_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_FULL_LENGTH_MAC(mac_alg)
}

pub unsafe fn PSA_ALG_AEAD_WITH_DEFAULT_LENGTH_TAG(aead_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_AEAD_WITH_DEFAULT_LENGTH_TAG(aead_alg)
}

pub unsafe fn PSA_ALG_AEAD_WITH_SHORTENED_TAG(
    aead_alg: psa_algorithm_t,
    tag_length: usize,
) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_AEAD_WITH_SHORTENED_TAG(aead_alg, tag_length)
}

pub unsafe fn PSA_ALG_HKDF(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_HKDF(hash_alg)
}

pub unsafe fn PSA_ALG_TLS12_PRF(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_TLS12_PRF(hash_alg)
}

pub unsafe fn PSA_ALG_TLS12_PSK_TO_MS(hash_alg: psa_algorithm_t) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_TLS12_PSK_TO_MS(hash_alg)
}

pub unsafe fn PSA_ALG_KEY_AGREEMENT(
    raw_key_agreement: psa_algorithm_t,
    key_derivation: psa_algorithm_t,
) -> psa_algorithm_t {
    psa_crypto_binding::shim_PSA_ALG_KEY_AGREEMENT(raw_key_agreement, key_derivation)
}

pub fn PSA_KEY_TYPE_IS_ECC_KEY_PAIR(key_type: psa_key_type_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_IS_ECC_KEY_PAIR(key_type) == 1 }
}

pub fn PSA_KEY_TYPE_IS_ECC_PUBLIC_KEY(key_type: psa_key_type_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_IS_ECC_PUBLIC_KEY(key_type) == 1 }
}

pub fn PSA_KEY_TYPE_IS_DH_KEY_PAIR(key_type: psa_key_type_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_IS_DH_KEY_PAIR(key_type) == 1 }
}

pub fn PSA_KEY_TYPE_IS_DH_PUBLIC_KEY(key_type: psa_key_type_t) -> bool {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_IS_DH_PUBLIC_KEY(key_type) == 1 }
}

pub fn PSA_KEY_TYPE_GET_CURVE(key_type: psa_key_type_t) -> psa_ecc_curve_t {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_GET_CURVE(key_type) }
}

pub fn PSA_KEY_TYPE_GET_GROUP(key_type: psa_key_type_t) -> psa_dh_group_t {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_GET_GROUP(key_type) }
}

pub fn PSA_KEY_TYPE_ECC_KEY_PAIR(curve: psa_ecc_curve_t) -> psa_key_type_t {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_ECC_KEY_PAIR(curve) }
}

pub fn PSA_KEY_TYPE_ECC_PUBLIC_KEY(curve: psa_ecc_curve_t) -> psa_key_type_t {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_ECC_PUBLIC_KEY(curve) }
}

pub fn PSA_KEY_TYPE_DH_KEY_PAIR(group: psa_dh_group_t) -> psa_key_type_t {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_DH_KEY_PAIR(group) }
}

pub fn PSA_KEY_TYPE_DH_PUBLIC_KEY(group: psa_dh_group_t) -> psa_key_type_t {
    unsafe { psa_crypto_binding::shim_PSA_KEY_TYPE_DH_PUBLIC_KEY(group) }
}

pub unsafe fn PSA_KEY_TYPE_PUBLIC_KEY_OF_KEY_PAIR(key_type: psa_key_type_t) -> psa_key_type_t {
    psa_crypto_binding::shim_PSA_KEY_TYPE_PUBLIC_KEY_OF_KEY_PAIR(key_type)
}

pub unsafe fn PSA_SIGN_OUTPUT_SIZE(
    key_type: psa_key_type_t,
    key_bits: usize,
    alg: psa_algorithm_t,
) -> usize {
    psa_crypto_binding::shim_PSA_SIGN_OUTPUT_SIZE(key_type, key_bits, alg)
}

pub unsafe fn PSA_ASYMMETRIC_ENCRYPT_OUTPUT_SIZE(
    key_type: psa_key_type_t,
    key_bits: usize,
    alg: psa_algorithm_t,
) -> usize {
    psa_crypto_binding::shim_PSA_ASYMMETRIC_ENCRYPT_OUTPUT_SIZE(key_type, key_bits, alg)
}

pub unsafe fn PSA_ASYMMETRIC_DECRYPT_OUTPUT_SIZE(
    key_type: psa_key_type_t,
    key_bits: usize,
    alg: psa_algorithm_t,
) -> usize {
    psa_crypto_binding::shim_PSA_ASYMMETRIC_DECRYPT_OUTPUT_SIZE(key_type, key_bits, alg)
}

pub unsafe fn PSA_EXPORT_KEY_OUTPUT_SIZE(key_type: psa_key_type_t, key_bits: usize) -> usize {
    psa_crypto_binding::shim_PSA_KEY_EXPORT_MAX_SIZE(key_type, key_bits)
}

pub fn PSA_HASH_LENGTH(alg: psa_algorithm_t) -> usize {
    unsafe { psa_crypto_binding::shim_PSA_HASH_LENGTH(alg) }
}

pub unsafe fn PSA_MAC_LENGTH(
    key_type: psa_key_type_t,
    key_bits: usize,
    alg: psa_algorithm_t,
) -> usize {
    psa_crypto_binding::shim_PSA_MAC_LENGTH(key_type, key_bits, alg)
}

pub unsafe fn PSA_MAC_TRUNCATED_LENGTH(alg: psa_algorithm_t) -> usize {
    // No longer in PSA spec but required to convert form psa_algorithm_to to algorithm/Mac
    psa_crypto_binding::shim_PSA_MAC_TRUNCATED_LENGTH(alg)
}

pub unsafe fn PSA_AEAD_ENCRYPT_OUTPUT_SIZE(alg: psa_algorithm_t, plaintext_bytes: usize) -> usize {
    psa_crypto_binding::shim_PSA_AEAD_ENCRYPT_OUTPUT_SIZE(alg, plaintext_bytes)
}

pub unsafe fn PSA_AEAD_DECRYPT_OUTPUT_SIZE(alg: psa_algorithm_t, ciphertext_bytes: usize) -> usize {
    psa_crypto_binding::shim_PSA_AEAD_DECRYPT_OUTPUT_SIZE(alg, ciphertext_bytes)
}
