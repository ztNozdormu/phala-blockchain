# Notest

## Fix missing C functions

```c++

// Orig:
void GFp_bn_mul_mont(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                     const BN_ULONG *np, const BN_ULONG *n0, size_t num);
// where
// - BN_ULONG is u32
// - size_t is u32

// Let's try:
void GFp_bn_mul_mont(uint32_t *rp, const uint32_t *ap, const uint32_t *bp,
                     const uint32_t *np, const uint32_t *n0, size_t num) {
  // nop
}
```




All missing functions:

```lisp
  (import "env" "ext_hashing_blake2_256_version_1" (func $ext_hashing_blake2_256_version_1 (type 9)))
  (import "env" "ext_hashing_twox_128_version_1" (func $ext_hashing_twox_128_version_1 (type 9)))
  (import "env" "ext_storage_set_version_1" (func $ext_storage_set_version_1 (type 10)))
  (import "env" "ext_storage_clear_version_1" (func $ext_storage_clear_version_1 (type 11)))
  (import "env" "ext_storage_root_version_1" (func $ext_storage_root_version_1 (type 12)))
  (import "env" "ext_crypto_ed25519_verify_version_1" (func $ext_crypto_ed25519_verify_version_1 (type 13)))
  (import "env" "ext_crypto_sr25519_verify_version_1" (func $ext_crypto_sr25519_verify_version_1 (type 13)))
  (import "env" "ext_storage_clear_prefix_version_1" (func $ext_storage_clear_prefix_version_1 (type 11)))
  (import "env" "ext_misc_print_utf8_version_1" (func $ext_misc_print_utf8_version_1 (type 11)))
  (import "env" "ext_misc_print_num_version_1" (func $ext_misc_print_num_version_1 (type 11)))
  (import "env" "ext_misc_print_hex_version_1" (func $ext_misc_print_hex_version_1 (type 11)))
  (import "env" "ext_crypto_sr25519_generate_version_1" (func $ext_crypto_sr25519_generate_version_1 (type 14)))
  (import "env" "ext_crypto_ed25519_generate_version_1" (func $ext_crypto_ed25519_generate_version_1 (type 14)))
  (import "env" "LIMBS_are_even" (func $LIMBS_are_even (type 1)))
  (import "env" "LIMBS_less_than_limb" (func $LIMBS_less_than_limb (type 0)))
  (import "env" "GFp_bn_neg_inv_mod_r_u64" (func $GFp_bn_neg_inv_mod_r_u64 (type 15)))
  (import "env" "LIMB_shr" (func $LIMB_shr (type 1)))
  (import "env" "LIMBS_shl_mod" (func $LIMBS_shl_mod (type 16)))
  (import "env" "LIMBS_less_than" (func $LIMBS_less_than (type 0)))
  (import "env" "LIMBS_are_zero" (func $LIMBS_are_zero (type 1)))
  (import "env" "ext_storage_get_version_1" (func $ext_storage_get_version_1 (type 15)))
  (import "env" "ext_storage_read_version_1" (func $ext_storage_read_version_1 (type 17)))
  (import "env" "ext_storage_changes_root_version_1" (func $ext_storage_changes_root_version_1 (type 8)))
  (import "env" "ext_storage_blake2_256_ordered_trie_root_version_1" (func $ext_storage_blake2_256_ordered_trie_root_version_1 (type 9)))
  (import "env" "ext_crypto_secp256k1_ecdsa_recover_compressed_version_1" (func $ext_crypto_secp256k1_ecdsa_recover_compressed_version_1 (type 18)))
  (import "env" "ext_allocator_malloc_version_1" (func $ext_allocator_malloc_version_1 (type 3)))
  (import "env" "ext_allocator_free_version_1" (func $ext_allocator_free_version_1 (type 7)))
```


## System Headers

```c
// from include/GFp/base.h
#include <stddef.h>  // supported
#include <stdint.h>  // no, maybe replaceable
// from internal.h
#include <stdalign.h>  // supported
```