#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const char *CString;

typedef struct CResult {
  bool is_err;
  CString error_msg;
  void *value;
} CResult;

typedef struct GlobalArgs {
  CString expiration;
  uint8_t code_hash[32];
  CString chain_id;
} GlobalArgs;

typedef struct Hashes {
  uint8_t (*ptr)[32];
  uintptr_t len;
} Hashes;

typedef struct RevealPk {
  void *_0;
} RevealPk;

struct CResult is_public_key_revealed(CString tendermint_addr, CString owner);

struct CResult new_reveal_pk(CString public_key, struct GlobalArgs args);

struct Hashes get_sign_bytes_reveal_pk(const struct RevealPk *reveal_pk_tx);

struct CResult attach_raw_signatures_reveal_pk(struct RevealPk reveal_pk_tx,
                                               CString public_key,
                                               CString signature);
