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

typedef struct DenominatedAmount {
  void *inner;
} DenominatedAmount;

typedef struct Tx {
  void *inner;
} Tx;

struct CResult query_native_token(CString tendermint_addr);

struct CResult is_public_key_revealed(CString tendermint_addr, CString owner);

struct CResult denominate_amount(CString tendermint_addr, uint64_t amount, CString token);

struct CResult new_reveal_pk(CString public_key, struct GlobalArgs args);

struct Hashes get_sign_bytes_reveal_pk(const struct RevealPk *reveal_pk_tx);

struct CResult attach_raw_signatures_reveal_pk(struct RevealPk reveal_pk_tx,
                                               CString public_key,
                                               CString signature);

struct CResult reveal_pk_attach_fee(struct RevealPk reveal_pk_tx,
                                    struct DenominatedAmount fee,
                                    CString token,
                                    CString fee_payer,
                                    uint64_t epoch,
                                    uint64_t gas_limit);

struct Hashes reveal_pk_get_fee_sign_bytes(const struct RevealPk *reveal_pk_tx);

struct CResult attach_fee_signature(struct RevealPk reveal_pk_tx,
                                    CString public_key,
                                    CString signature);

struct CResult reveal_pk_validate_tx(const struct RevealPk *reveal_pk);

struct Tx reveal_pk_payload(struct RevealPk reveal_pk_tx);

struct CResult broadcast_tx(CString tendermint_addr, struct Tx tx);
