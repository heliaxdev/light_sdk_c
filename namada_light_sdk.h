#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using CString = const char*;

struct CResult {
  bool is_err;
  CString error_msg;
  void *value;
};

struct GlobalArgs {
  CString expiration;
  uint8_t code_hash[32];
  CString chain_id;
};

struct Hashes {
  uint8_t (*ptr)[32];
  uintptr_t len;
};

struct RevealPk {
  void *_0;
};

extern "C" {

CResult new_reveal_pk(CString public_key, GlobalArgs args);

Hashes get_sign_bytes_reveal_pk(const RevealPk *reveal_pk_tx);

CResult attach_raw_signatures_reveal_pk(RevealPk reveal_pk_tx,
                                        CString public_key,
                                        CString signature);

} // extern "C"
