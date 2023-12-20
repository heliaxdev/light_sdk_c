#include <cstring>
extern "C" {
    #include "./namada_light_sdk.h"
}
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <iostream>
#include <istream>

using namespace std;

int main(int argc, char** argv) {
    uint8_t code_hash[32];
    std::fill(code_hash, code_hash + 32, 0);

    struct GlobalArgs test_args = {
            .expiration = "",
            .code_hash = *code_hash,
            .chain_id = "namada-internal.000000000000-0"
    };

    if (strcmp(test_args.expiration, "") == 0) {
        cout << "Expiration: None" << endl;
    } else {
        cout << "Expiration: " << test_args.expiration << endl;
    }
    cout << "Code hash: ";
    for (int i = 0; i < 32 ; ++i) {
        printf("%d", test_args.code_hash[i]);
    }
    cout << endl;
    cout << "Chain Id: " << test_args.chain_id << endl;

    CString pubkey = "tpknam1qpyfnrl6qdqvguah9kknvp9t6ajcrec7fge56pcgaa655zkua3nds48x83t";

    CResult result = new_reveal_pk(pubkey, test_args);
    if (result.is_err) {
        cout << "Error: " << result.error_msg << endl;
    } else {
        cout << "Result: Ok" << endl;
    }
    const RevealPk* reveal = static_cast<RevealPk*>(result.value);

    Hashes hashes = get_sign_bytes_reveal_pk(reveal);
    cout << "Bytes to sign: ";
    for (int i = 0; i < 32 ; ++i) {
        printf("%d", hashes.ptr[0][i]);
    }
    cout << endl;
    CString signature = "signam1qpftxz3f6qeucmz93nfr2h3n7kyefagswjqrtk38cgnmgv6kszyp97mjy2mkv8jgyjs7frpuu9lncp97jgjvpnsvk78gx7w9q9psr3s97r3d8w";
    CResult signed_tx = attach_raw_signatures_reveal_pk(
            *reveal,
            "tpknam1qpyfnrl6qdqvguah9kknvp9t6ajcrec7fge56pcgaa655zkua3nds48x83t",
            signature);
    if (signed_tx.is_err) {
        cout << "Error: " << signed_tx.error_msg << endl;
    } else {
        cout << "Result: Ok" << endl;
    }

    CResult is_pk_revealed = is_public_key_revealed("0.0.0.0:26657", "tnam1qxfj3sf6a0meahdu9t6znp05g8zx4dkjtgyn9gfu");
    if (is_pk_revealed.is_err) {
        cout << "Error: " << is_pk_revealed.error_msg << endl;
    } else {
        cout << "Result: " << *static_cast<bool*>(is_pk_revealed.value) << endl;
    }
}