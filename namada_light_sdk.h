#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Tx {
  void *inner;
};

struct RevealPk {
  Tx _0;
};

using CString = const char*;

template<typename T>
struct CResult {
  enum class Tag {
    Ok,
    Err,
  };

  struct Ok_Body {
    T _0;
  };

  struct Err_Body {
    CString _0;
  };

  Tag tag;
  union {
    Ok_Body ok;
    Err_Body err;
  };
};

struct GlobalArgs {
  CString expiration;
  uint8_t code_hash[32];
  CString chain_id;
};

extern "C" {

CResult<RevealPk> new_reveal_pk(CString public_key, GlobalArgs args);

} // extern "C"
