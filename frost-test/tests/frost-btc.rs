use bitcoin::consensus::{Decodable, Encodable};
use bitcoin::hashes::sha256;
use bitcoin::psbt::serialize::Serialize;
use bitcoin::secp256k1::ecdsa::SerializedSignature;
use bitcoin::secp256k1::rand::thread_rng;
use bitcoin::secp256k1::{rand, Message, Secp256k1};
use bitcoin::util::key;
use bitcoin::{
    EcdsaSighashType, KeyPair, Network, OutPoint, PackedLockTime, PrivateKey, PublicKey,
    SchnorrSighashType, Script, Transaction, XOnlyPublicKey,
};
use ctrlc::Signal;
use frost_test::bitcoind;
use frost_test::bitcoind::stop_pid;
use hashbrown::HashMap;
use nix::libc::pid_t;
use nix::sys::signal;
use nix::unistd::Pid;
use rand_core::OsRng;
use std::ffi::c_uint;
use std::iter::Map;
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use ureq::serde_json;
use ureq::serde_json::Value;
use wtfrost::common::{PolyCommitment, Signature};
use wtfrost::errors::AggregatorError;
use wtfrost::{
    bip340::{
        test_helpers::{dkg, sign},
        Error as Bip340Error, SchnorrProof,
    },
    common::PublicNonce,
    traits::Signer,
    v1::{self, SignatureAggregator},
    Point,
};

const BITCOIND_URL: &str = "http://abcd:abcd@localhost:18443";

#[test]
fn blog_post() {
    // https://medium.com/coinmonks/creating-and-signing-a-segwit-transaction-from-scratch-ec98577b526a
    let secp = bitcoin::secp256k1::Secp256k1::new();

    let secret_bytes =
        hex::decode("26F85CE8B2C635AD92F6148E4443FE415F512F3F29F44AB0E2CBDA819295BBD5").unwrap();
    let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&secret_bytes).unwrap();
    let secp_public_key = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    let private_key = bitcoin::PrivateKey::new(secret_key, bitcoin::Network::Testnet);
    let public_key = bitcoin::PublicKey::from_private_key(&secp, &private_key);
    let address = bitcoin::Address::p2wpkh(&public_key, bitcoin::Network::Testnet).unwrap();
    println!(
        "address {} public_key {} {}",
        address,
        public_key,
        address.script_pubkey()
    );

    let blog_tx_bytes = hex::decode("02000000000103ed204affc7519dfce341db0569687569d12b1520a91a9824531c038ad62aa9d1010000006a47304402200da2c4d8f2f44a8154fe127fe5bbe93be492aa589870fe77eb537681bc29c8ec02201eee7504e37db2ef27fa29afda46b6c331cd1a651bb6fa5fd85dcf51ac01567a01210242BF11B788DDFF450C791F16E83465CC67328CA945C703469A08E37EF0D0E061ffffffff9cb872539fbe1bc0b9c5562195095f3f35e6e13919259956c6263c9bd53b20b70100000000ffffffff8012f1ec8aa9a63cf8b200c25ddae2dece42a2495cc473c1758972cfcd84d90401000000171600146a721dcca372f3c17b2c649b2ba61aa0fda98a91ffffffff01b580f50000000000160014cb61ee4568082cb59ac26bb96ec8fbe0109a4c000002483045022100f8dac321b0429798df2952d086e763dd5b374d031c7f400d92370ae3c5f57afd0220531207b28b1b137573941c7b3cf5384a3658ef5fc238d26150d8f75b2bcc61e70121025972A1F2532B44348501075075B31EB21C02EEF276B91DB99D30703F2081B7730247304402204ebf033caf3a1a210623e98b49acb41db2220c531843106d5c50736b144b15aa02201a006be1ebc2ffef0927d4458e3bb5e41e5abc7e44fc5ceb920049b46f879711012102AE68D299CBB8AB99BF24C9AF79A7B13D28AC8CD21F6F7F750300EDA41A589A5D00000000").unwrap();
    let transaction =
        bitcoin::Transaction::consensus_decode(&mut blog_tx_bytes.as_slice()).unwrap();
    println!("Blog Post tx {:?}", transaction);

    let mut transaction_bytes = vec![];
    transaction
        .consensus_encode(&mut transaction_bytes)
        .unwrap();
    assert_eq!(blog_tx_bytes, transaction_bytes);
    println!(
        "tx.input[1].witness ({} rows) {} {}",
        transaction.input[1].witness.len(),
        hex::encode(transaction.input[1].witness.second_to_last().unwrap()),
        hex::encode(transaction.input[1].witness.last().unwrap())
    );

    let segwit_signing_input_script_pubkey = address.script_pubkey().p2wpkh_script_code().unwrap();

    println!(
        "sighash input #{} script_pubkey {} value {}",
        1, &segwit_signing_input_script_pubkey, 9300
    );

    let mut comp = bitcoin::util::sighash::SighashCache::new(&transaction);
    let segwit_sighash = comp
        .segwit_signature_hash(
            1,
            &segwit_signing_input_script_pubkey,
            9300,
            EcdsaSighashType::All,
        )
        .unwrap();
    println!(
        "calc sighash len {} {}",
        segwit_sighash.len(),
        hex::encode(segwit_sighash.to_vec())
    );
    let blog_sighash_bytes =
        hex::decode("4876161197833dd58a1a2ba20728633677f38b9a7513a4d7d3714a7f7d3a1fa2").unwrap();
    println!(
        "blog sighash len {} {}",
        blog_sighash_bytes.len(),
        hex::encode(&blog_sighash_bytes)
    ); // second sha256
    assert_eq!(segwit_sighash.to_vec(), blog_sighash_bytes);

    let user_utxo_msg = Message::from_slice(&segwit_sighash).unwrap();
    let user_utxo_segwit_sig = secp.sign_ecdsa_low_r(&user_utxo_msg, &secret_key);
    let user_utxo_segwit_sig_bytes = user_utxo_segwit_sig.serialize_der();
    let finalized = [
        user_utxo_segwit_sig_bytes.as_ref(),
        &[EcdsaSighashType::All.to_u32() as u8],
    ]
    .concat();

    println!(
        "CALC SIG ({}) {}",
        user_utxo_segwit_sig_bytes.len(),
        hex::encode(&user_utxo_segwit_sig_bytes)
    );
    let calc_verify = secp.verify_ecdsa(&user_utxo_msg, &user_utxo_segwit_sig, &secp_public_key);
    assert!(calc_verify.is_ok(), "calc sig check {:?}", calc_verify);

    // libsecp verify only works on "low_r" 70 byte signatures
    // while this doesnt match the blog post, it is a sig of the same data, re-running openssl unil the result is short/low
    let blog_post_good_sig_bytes = hex::decode("30440220492eae58ddf8c2f8f1ab5b2b2c45432902a3c2dda508bf79319b3fde26e1364a022078bbdde1b79410efc07b19a64038242525883a94de3079668308aa45b035a6d8").unwrap();
    println!(
        "BLOG SIG ({}) {}",
        blog_post_good_sig_bytes.len(),
        hex::encode(&blog_post_good_sig_bytes)
    );
    let blog_sig =
        bitcoin::secp256k1::ecdsa::Signature::from_der(&blog_post_good_sig_bytes).unwrap();
    let blog_verify = secp.verify_ecdsa(&user_utxo_msg, &blog_sig, &secp_public_key);
    // https://docs.rs/secp256k1/0.24.1/src/secp256k1/ecdsa/mod.rs.html#400
    assert!(blog_verify.is_ok(), "blog sig check {:?}", blog_verify);
}

#[test]
fn frost_btc() {
    // Singer setup
    let threshold = 3;
    let total = 4;
    let mut rng = OsRng::default();
    let mut signers = [
        v1::Signer::new(&[0, 1], total, threshold, &mut rng),
        v1::Signer::new(&[2], total, threshold, &mut rng),
        v1::Signer::new(&[3], total, threshold, &mut rng),
    ];

    // DKG (Distributed Key Generation)
    let (public_key_shares, group_public_key) = dkg_round(&mut rng, &mut signers);

    let peg_wallet_lobby_address = bitcoin::PublicKey::from_slice(&[0; 33]);

    // Peg Wallet Address from group key
    let peg_wallet_address =
        bitcoin::PublicKey::from_slice(&group_public_key.compress().as_bytes()).unwrap();

    // bitcoind regtest
    let bitcoind_pid = bitcoind::bitcoind_setup();

    // create user keys
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let user_secret_key = bitcoin::secp256k1::SecretKey::new(&mut rand::thread_rng());
    let user_secp_public_key =
        bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &user_secret_key);
    let user_private_key = bitcoin::PrivateKey::new(user_secret_key, bitcoin::Network::Regtest);
    let user_public_key = bitcoin::PublicKey::from_private_key(&secp, &user_private_key);
    let user_address =
        bitcoin::Address::p2wpkh(&user_public_key, bitcoin::Network::Regtest).unwrap();
    println!(
        "user public key {} serialized {} witness hash {:?} p2wpkh signing script {}",
        user_address,
        hex::encode(user_public_key.serialize()),
        user_public_key.wpubkey_hash().unwrap(),
        user_address.script_pubkey().p2wpkh_script_code().unwrap()
    );

    // mine block to create btc
    let result = bitcoind_mine(&user_public_key.serialize().try_into().unwrap());
    let block_id = result
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .as_str()
        .unwrap();
    println!("mined block_id {:?}", block_id);
    let result = bitcoind_rpc("getblock", [block_id]);
    let block = result.as_object().unwrap();
    let txid = block.get("tx").unwrap().get(0).unwrap().as_str().unwrap();
    println!("mined txid {:?}", txid);
    let result = bitcoind_rpc("getrawtransaction", (txid, false, block_id));
    let user_funding_transaction_bytes_hex = result.as_str().unwrap();
    let _ = bitcoind_rpc(
        "decoderawtransaction",
        [&user_funding_transaction_bytes_hex],
    );

    // Peg in to stx address
    let stx_address = [0; 32];
    let user_funding_transaction = bitcoin::Transaction::consensus_decode(
        &mut hex::decode(user_funding_transaction_bytes_hex)
            .unwrap()
            .as_slice(),
    )
    .unwrap();
    println!(
        "funding tx txid {} wtxid {}",
        user_funding_transaction.txid(),
        user_funding_transaction.wtxid()
    );
    println!("funding tx {:?}", user_funding_transaction);

    let funding_utxo = &user_funding_transaction.output[0];
    println!(
        "funding UTXO with {:?} sats utxo.script_pub_key: {}",
        funding_utxo.value,
        funding_utxo.script_pubkey.asm()
    );
    let mut peg_in = build_peg_in_op_return(
        funding_utxo.value - 1000,
        peg_wallet_address,
        stx_address,
        &user_funding_transaction,
        0,
    );
    let peg_in_sighash_pubkey_script = user_address.script_pubkey().p2wpkh_script_code().unwrap();
    let mut comp = bitcoin::util::sighash::SighashCache::new(&peg_in);
    let peg_in_sighash = comp
        .segwit_signature_hash(
            0,
            &peg_in_sighash_pubkey_script,
            funding_utxo.value,
            EcdsaSighashType::All,
        )
        .unwrap();
    let peg_in_msg = Message::from_slice(&peg_in_sighash).unwrap();
    let peg_in_sig = secp.sign_ecdsa_low_r(&peg_in_msg, &user_secret_key);
    let peg_in_verify = secp.verify_ecdsa(&peg_in_msg, &peg_in_sig, &user_secp_public_key);
    assert!(peg_in_verify.is_ok());
    //let (peg_in_step_a, peg_in_step_b) = two_phase_peg_in(peg_wallet_address, stx_address, user_utxo);
    peg_in.input[0]
        .witness
        .push_bitcoin_signature(&peg_in_sig.serialize_der(), EcdsaSighashType::All);
    peg_in.input[0].witness.push(user_public_key.serialize());
    let mut peg_in_bytes: Vec<u8> = vec![];
    peg_in.consensus_encode(&mut peg_in_bytes).unwrap();

    let mut consensus_check_funding_out0: Vec<u8> = vec![];
    user_funding_transaction.output[0]
        .script_pubkey
        .consensus_encode(&mut consensus_check_funding_out0)
        .unwrap();

    println!(
        "peg-in (OP_RETURN) tx id {} signing txin pubkey script {}",
        peg_in.txid(),
        peg_in_sighash_pubkey_script.asm()
    );
    let peg_in_bytes_hex = hex::encode(&peg_in_bytes);
    let _ = bitcoind_rpc("decoderawtransaction", [&peg_in_bytes_hex]);
    println!("peg-IN tx bytes {}", peg_in_bytes_hex);
    let peg_in_result_value = bitcoind_rpc("sendrawtransaction", [&peg_in_bytes_hex]);
    let peg_in_result = peg_in_result_value.as_object().unwrap();
    println!("{:?}", peg_in_result);
    assert!(
        !peg_in_result.contains_key("error"),
        "{}",
        peg_in_result.get("error").unwrap().get("message").unwrap()
    );

    // Peg out to btc address
    let peg_in_utxo = OutPoint {
        txid: peg_in.txid(),
        vout: 0,
    };
    let mut peg_out = build_peg_out(funding_utxo.value - 1000, user_public_key, peg_in_utxo);
    let mut peg_out_bytes: Vec<u8> = vec![];
    let _peg_out_bytes_len = peg_out.consensus_encode(&mut peg_out_bytes).unwrap();

    let sighash = peg_out.signature_hash(
        0,
        &peg_in.output[0].script_pubkey,
        SchnorrSighashType::All as u32,
    );
    let signing_payload = sighash.as_hash().to_vec();

    // signing. Signers: 0 (parties: 0, 1) and 1 (parties: 2)
    let schnorr_proof = signing_round(
        &signing_payload,
        threshold,
        total,
        &mut rng,
        &mut signers,
        public_key_shares,
    )
    .unwrap();

    let mut sig_bytes = vec![];

    sig_bytes.extend(schnorr_proof.r.to_bytes());
    sig_bytes.extend(schnorr_proof.s.to_bytes());

    peg_out.input[0].witness.push(&sig_bytes);
    peg_out.input[0]
        .witness
        .push(&group_public_key.x().to_bytes());

    let peg_out_bytes_hex = hex::encode(&peg_out_bytes);

    println!("peg-OUT tx bytes {}", &peg_out_bytes_hex);

    bitcoind_rpc("sendrawtransaction", [&peg_out_bytes_hex]);

    stop_pid(bitcoind_pid);
}

fn bitcoind_rpc(method: &str, params: impl ureq::serde::Serialize) -> serde_json::Value {
    let rpc = ureq::json!({"jsonrpc": "1.0", "id": "tst", "method": method, "params": params});
    match ureq::post(BITCOIND_URL).send_json(&rpc) {
        Ok(response) => {
            let status = response.status();
            let json = response.into_json::<serde_json::Value>().unwrap();
            let result = json.as_object().unwrap().get("result").unwrap().clone();
            println!("{} -> {}", rpc.to_string(), result.to_string());
            result
        }
        Err(err) => {
            let json = err
                .into_response()
                .unwrap()
                .into_json::<serde_json::Value>()
                .unwrap();
            let err = json.as_object().unwrap().get("error").unwrap();
            println!("{} -> {}", rpc.to_string(), err.to_string());
            json
        }
    }
}

fn bitcoind_mine(public_key_bytes: &[u8; 33]) -> Value {
    let public_key = bitcoin::PublicKey::from_slice(public_key_bytes).unwrap();
    let address = bitcoin::Address::p2wpkh(&public_key, bitcoin::Network::Regtest).unwrap();
    bitcoind_rpc("generatetoaddress", (100, address.to_string()))
}

fn build_peg_in_op_return(
    satoshis: u64,
    peg_wallet_address: bitcoin::PublicKey,
    stx_address: [u8; 32],
    utxo: &Transaction,
    utxo_vout: u32,
) -> Transaction {
    let utxo_point = OutPoint {
        txid: utxo.txid(),
        vout: utxo_vout,
    };
    let witness = bitcoin::blockdata::witness::Witness::new();
    let peg_in_input = bitcoin::TxIn {
        previous_output: utxo_point,
        script_sig: Default::default(),
        sequence: bitcoin::Sequence(0xFFFFFFFF),
        witness: witness,
    };
    let mut sip_21_peg_in_data = vec![0, 0, '<' as u8];
    sip_21_peg_in_data.extend_from_slice(&stx_address);
    let op_return = Script::new_op_return(&sip_21_peg_in_data);
    let peg_in_output_0 = bitcoin::TxOut {
        value: 0,
        script_pubkey: op_return,
    };
    let secp = bitcoin::util::key::Secp256k1::new();
    // crate type weirdness
    let peg_wallet_address_secp =
        bitcoin::secp256k1::PublicKey::from_slice(&peg_wallet_address.to_bytes()).unwrap();
    let taproot = Script::new_v1_p2tr(&secp, XOnlyPublicKey::from(peg_wallet_address_secp), None);
    let peg_in_output_1 = bitcoin::TxOut {
        value: satoshis,
        script_pubkey: taproot,
    };
    bitcoin::blockdata::transaction::Transaction {
        version: 2,
        lock_time: PackedLockTime(0),
        input: vec![peg_in_input],
        output: vec![peg_in_output_0, peg_in_output_1],
    }
}

fn two_phase_peg_in(
    peg_wallet_address: PublicKey,
    stx_address: [u8; 32],
    user_utxo: OutPoint,
) -> (Transaction, Transaction) {
    let peg_in_step_a = build_peg_in_step_a(1000, peg_wallet_address, stx_address, user_utxo);
    let mut peg_in_step_a_bytes: Vec<u8> = vec![];
    peg_in_step_a
        .consensus_encode(&mut peg_in_step_a_bytes)
        .unwrap();
    println!("peg-in step A tx");
    println!("{:?}", hex::encode(&peg_in_step_a_bytes));

    let peg_in_step_b = build_peg_in_step_b(&peg_in_step_a, peg_wallet_address);
    let mut peg_in_step_b_bytes: Vec<u8> = vec![];
    peg_in_step_b
        .consensus_encode(&mut peg_in_step_b_bytes)
        .unwrap();
    println!("peg-in step B tx");
    println!("{:?}", hex::encode(&peg_in_step_b_bytes));
    (peg_in_step_a, peg_in_step_b)
}

fn build_peg_in_step_a(
    satoshis: u64,
    peg_wallet_lobby_address: bitcoin::PublicKey,
    stx_address: [u8; 32],
    utxo: OutPoint,
) -> Transaction {
    // Peg-In TX
    // crate type weirdness
    let peg_wallet_lobby_address_secp =
        bitcoin::secp256k1::PublicKey::from_slice(&peg_wallet_lobby_address.to_bytes()).unwrap();
    let lobby_tx_out = Script::new_v0_p2wpkh(
        &bitcoin::PublicKey::new(peg_wallet_lobby_address_secp)
            .wpubkey_hash()
            .unwrap(),
    );
    let peg_in_input = bitcoin::TxIn {
        previous_output: utxo,
        script_sig: lobby_tx_out.p2wpkh_script_code().unwrap(),
        sequence: Default::default(),
        witness: Default::default(),
    };
    let p2wpk = Script::new_v0_p2wpkh(&peg_wallet_lobby_address.wpubkey_hash().unwrap());
    let peg_in_output = bitcoin::TxOut {
        value: satoshis,
        script_pubkey: p2wpk,
    };
    bitcoin::blockdata::transaction::Transaction {
        version: 0,
        lock_time: PackedLockTime(0),
        input: vec![peg_in_input],
        output: vec![peg_in_output],
    }
}

fn build_peg_in_step_b(
    step_a: &Transaction,
    peg_wallet_address: bitcoin::PublicKey,
) -> Transaction {
    let peg_in_outpoint = OutPoint {
        txid: step_a.txid(),
        vout: 0,
    };
    let peg_out_input = bitcoin::TxIn {
        previous_output: peg_in_outpoint,
        script_sig: Default::default(),
        sequence: Default::default(),
        witness: Default::default(),
    };
    // crate type weirdness
    let peg_wallet_address_secp =
        bitcoin::secp256k1::PublicKey::from_slice(&peg_wallet_address.to_bytes()).unwrap();
    let secp = bitcoin::util::key::Secp256k1::new();
    let taproot = Script::new_v1_p2tr(&secp, XOnlyPublicKey::from(peg_wallet_address_secp), None);
    let peg_out_output = bitcoin::TxOut {
        value: step_a.output[0].value,
        script_pubkey: taproot,
    };
    bitcoin::blockdata::transaction::Transaction {
        version: 0,
        lock_time: PackedLockTime(0),
        input: vec![peg_out_input],
        output: vec![peg_out_output],
    }
}

fn build_peg_out(satoshis: u64, user_address: bitcoin::PublicKey, utxo: OutPoint) -> Transaction {
    let peg_out_input = bitcoin::TxIn {
        previous_output: utxo,
        script_sig: Default::default(),
        sequence: Default::default(),
        witness: Default::default(),
    };
    let p2wpk = Script::new_v0_p2wpkh(&user_address.wpubkey_hash().unwrap());
    let peg_out_output = bitcoin::TxOut {
        value: satoshis,
        script_pubkey: p2wpk,
    };
    bitcoin::blockdata::transaction::Transaction {
        version: 2,
        lock_time: PackedLockTime(0),
        input: vec![peg_out_input],
        output: vec![peg_out_output],
    }
}

fn signing_round(
    message: &[u8],
    threshold: usize,
    total: usize,
    mut rng: &mut OsRng,
    signers: &mut [v1::Signer; 3],
    public_commitments: Vec<PolyCommitment>,
) -> Result<SchnorrProof, Bip340Error> {
    // decide which signers will be used
    let mut signers = [signers[0].clone(), signers[1].clone()];

    let (nonces, shares) = sign(message, &mut signers, rng);

    let sig = SignatureAggregator::new(total, threshold, public_commitments.clone())
        .unwrap()
        .sign(&message, &nonces, &shares)
        .unwrap();

    SchnorrProof::new(&sig)
}

fn dkg_round(
    mut rng: &mut OsRng,
    signers: &mut [v1::Signer; 3],
) -> (Vec<PolyCommitment>, wtfrost::Point) {
    let polys = dkg(signers, rng).unwrap();
    let pubkey = polys.iter().fold(Point::new(), |s, poly| s + poly.A[0]);
    (polys, pubkey)
}
