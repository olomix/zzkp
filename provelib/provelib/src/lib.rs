use methods::{
    GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID
};

use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use std::ffi::{c_char, CStr};

#[no_mangle]
pub unsafe extern "C" fn prove_pow(x: u32, y: u32, resp: *mut *mut c_char) -> bool {
    if resp.is_null() {
        panic!("resp is null");
    }

    let env = ExecutorEnv::builder()
        .write(&x)
        .unwrap()
        .write(&y)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    println!("prover name: {}, len: {}", prover.get_name(),
             GUEST_CODE_FOR_ZK_PROOF_ELF.len());

    let prove_info = prover
        .prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF);

    println!("got some result");

    let prove_info = match prove_info {
        Ok(prove_info) => {
            prove_info
        }
        Err(e) => {
            println!("Prove error: {:?}", e);
            return false;
        }
    };

    let receipt = prove_info.receipt;

    let receipt_json = serde_json::to_string(&receipt).unwrap();
    // println!("Receipt: {:?}", receipt_json);

    *resp = libc::malloc(receipt_json.len() + 1) as *mut c_char;

    println!("addr: {:?}, len: {}", *resp, receipt_json.len());

    receipt_json.as_bytes().iter().enumerate().for_each(|(i, &c)| {
        *(*resp).offset(i as isize) = c as c_char;
    });
    *(*resp).offset(receipt_json.len() as isize) = 0;

    true
}

#[no_mangle]
pub extern "C" fn receipt_verify(receipt: *const c_char) -> bool {
    if receipt.is_null() {
        panic!("receipt is null");
    }
    let receipt = unsafe { CStr::from_ptr(receipt) };
    let receipt: Receipt = serde_json::from_slice(receipt.to_bytes()).unwrap_or_else(|_| {
        panic!("Failed to parse receipt");
    });

    return receipt
        .verify(GUEST_CODE_FOR_ZK_PROOF_ID)
        .is_ok();
}

