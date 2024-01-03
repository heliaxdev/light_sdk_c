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
#include <string>

using namespace std;

// Give a prompt to the user, read the input, and return it
CString read_input(CString prompt) {
    string output;
    cout << prompt;
    cin >> output;
    char * str = new char [output.length()+1];
    std::strcpy(str, output.c_str());
    return str;
}

// Query if a given public key has been revealed on chain.
// The program exits if it has or the query errors
void query_if_pk_revealed(const CString rpc, const CString address) {
    CResult is_pk_revealed = is_public_key_revealed(rpc, address);
     bool query_result;
     if (is_pk_revealed.is_err) {
         cout << "Error: " << is_pk_revealed.error_msg << endl;
         std::exit(1);
     } else {
         query_result = *static_cast<bool*>(is_pk_revealed.value);
         if (query_result) {
             cout << "This public key has already been revealed." << query_result << endl;
             std::exit(0);
         } else {
            cout << "This public key has not been revealed. Proceeding.." << endl;
         }
     }
}

// Construct the basic tx to reveal the public key. Print out the bytes
// that must be signed before returning.
RevealPk construct_reveal_pk_tx(const CString chain_id, const CString pubkey) {
    uint8_t code_hash[32];
    std::fill(code_hash, code_hash + 32, 0);
    struct GlobalArgs test_args = {
            .expiration = "",
            .code_hash = *code_hash,
            .chain_id = chain_id
    };

    // Construct the tx to reveal this public key
    CResult reveal_result = new_reveal_pk(pubkey, test_args);
    if (reveal_result.is_err) {
        cout << "Error: " << reveal_result.error_msg << endl;
        std::exit(1);
    } else {
        cout << "The base reveal tx has been constructed. Proceeding..." << endl;
    }
    const RevealPk* reveal = static_cast<RevealPk*>(reveal_result.value);

    // Print the bytes of this tx to be signed
    Hashes hashes = get_sign_bytes_reveal_pk(reveal);
    cout << "===============" << endl;
    cout << "Tx bytes to sign: [";
    for (int i = 0; i < 32 ; ++i) {
        printf("%d, ", hashes.ptr[0][i]);
    }
    cout << "]" << endl << "===============" << endl << endl;
    return *reveal;
}

// Attaches a signature to a tx that reveals a publick key
RevealPk attach_signature(const RevealPk reveal, const CString pk) {
    CString signature = read_input("Please input the signature of the above bytes: ");
    CResult signed_tx = attach_raw_signatures_reveal_pk(
        reveal,
        pk,
        signature
    );
    if (signed_tx.is_err) {
        cout << "Error: " << signed_tx.error_msg << endl;
        std::exit(1);
    } else {
        cout << "Attached signature to the tx. Proceeding..." << endl;
    }
     return *static_cast<RevealPk*>(signed_tx.value);
}

// Attach a gas fee to a reveal pk tx
RevealPk attach_fee(
    const RevealPk reveal,
    const CString rpc,
    const CString pk,
    const uint64_t fee_amount,
    const uint64_t gas_limit,
    const CString token_address,
    const uint64_t epoch
) {
    // first we correctly format the amount depending on how
    // many decimal places the fee token uses.
    CResult fee_amount_result = denominate_amount(
         rpc,
         fee_amount,
         token_address
    );
    DenominatedAmount amount;
    if (fee_amount_result.is_err) {
        cout << "Error: " << fee_amount_result.error_msg << endl;
        std::exit(1);
    } else {
        amount = *static_cast<DenominatedAmount*>(fee_amount_result.value);
    }

    // We attach the fee to the tx and return
    CResult attached_fees = reveal_pk_attach_fee(
        reveal,
        amount,
        token_address,
        pk,
        epoch,
        gas_limit
    );

    if (attached_fees.is_err) {
        cout << "Error: " << attached_fees.error_msg << endl;
        std::exit(1);
    } else {
        cout << "Attached fees to the tx. Proceeding..." << endl;
    }

    const RevealPk* reveal_tx =  static_cast<RevealPk*>(attached_fees.value);
     // Print the bytes of the fee data to be signed
    Hashes hashes = reveal_pk_get_fee_sign_bytes(reveal_tx);
    cout << "===============" << endl;
    cout << "Fee data bytes to sign: [";
    for (int i = 0; i < 32 ; ++i) {
        printf("%d, ", hashes.ptr[0][i]);
    }
    cout << "]" << endl << "===============" << endl << endl;
    return *reveal_tx;
}

RevealPk attach_fee_signature( const RevealPk reveal, const CString pubkey) {
    CString signature = read_input("Please input the signature of the above bytes: ");
    CResult result = attach_fee_signature(reveal, pubkey, signature);
    RevealPk fee_signed;
    if (result.is_err) {
        cout << "Error: " << result.error_msg << endl;
        std::exit(1);
    } else {
        cout << "Fee signature attached to tx. Proceeding..." << endl;
    }
    return *static_cast<RevealPk*>(result.value);
}

int main(int argc, char** argv) {
    // user inputs
    // 0.0.0.0:27657
    CString rpc = read_input("Please input the rpc address: ");
    // "e2e-test.21f7be21bdd6b0cdeeb9e"
    CString chain_id = read_input("Please input the chain id: ");
    // "tnam1qxfj3sf6a0meahdu9t6znp05g8zx4dkjtgyn9gfu"
    CString address = read_input("Please input the address whose public key you wish to reveal: ");

    // Query if this public key has already been revealed. It is expected that it has not.
    query_if_pk_revealed(rpc, address);

    // "tpknam1qrnw8mxyqlj60mykgevnldcj5mg2fya7fs5a8xqdkd2gwtxhef0zy8a2wha"
    // The public key to be revealed
    CString pubkey = read_input("Please input the public key: ");

    // get the native token address
    CResult native_token_query = query_native_token(rpc);
    if (native_token_query.is_err) {
        cout << "Error: " << native_token_query.error_msg << endl;
        std::exit(1);
    }
    CString native_token_addr = *static_cast<CString*>(native_token_query.value);
    cout << "Native token address: " << native_token_addr << endl;

    // Construct the base tx
    RevealPk reveal_pk_tx = construct_reveal_pk_tx(chain_id, pubkey);

    // Attach the signature to the tx
    reveal_pk_tx = attach_signature(
        reveal_pk_tx,
        pubkey
    );

    // Attach the gas fees to the tx
    reveal_pk_tx = attach_fee(
        reveal_pk_tx,
        rpc,
        pubkey,
        100,
        10000,
        native_token_addr,
        0
    );

    reveal_pk_tx = attach_fee_signature(
        reveal_pk_tx,
        pubkey
    );

    CResult validated = reveal_pk_validate_tx(&reveal_pk_tx);
    if (validated.is_err) {
        cout << "Error: " << validated.error_msg << endl;
    } else {
        cout << "This transaction is valid." << endl;
    }

    Tx tx = reveal_pk_payload(reveal_pk_tx);
    CResult result = broadcast_tx(rpc, tx);
    if (result.is_err) {
        cout << "Error: " << result.error_msg << endl;
    } else {
        cout << "Tx submitted successfully" << endl;
    }
}