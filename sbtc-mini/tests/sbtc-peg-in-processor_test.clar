(define-constant err-burn-tx-already-processed (err u2000))

(define-constant err-peg-in-expired (err u4000))
(define-constant err-not-a-peg-wallet (err u4001))
(define-constant err-invalid-spending-pubkey (err u4003))
(define-constant err-peg-value-not-found (err u4005))
(define-constant err-missing-witness (err u4006))
(define-constant err-unlock-script-not-found-or-invalid (err u4007))

(define-constant err-script-invalid-opcode (err u4010))
(define-constant err-script-invalid-version (err u4011))
(define-constant err-script-not-op-drop (err u4012))
(define-constant err-script-checksig-missing (err u4013))
(define-constant err-script-missing-pubkey (err u4014))
(define-constant err-script-invalid-principal (err u4015))
(define-constant err-script-invalid-length (err u4016))


(define-constant wallet-1 'ST1SJ3DTE5DN7X54YDH5D64R3BCB6A2AG2ZQ8YPD5)
(define-constant wallet-1-pubkey 0x03cd2cfdbd2ad9332828a7a13ef62cb999e063421c708e863a7ffed71fb61c88c9)

;; https://github.com/hirosystems/stacks.js/blob/c9e420e521cdc02d7ec81ea082f62d0a2d6c5e27/packages/stacking/src/constants.ts#L2

;; P2WPKH
;; WIF private key cVqZm6SNztZsZC75wAhmkewxxhCehq2QL7S8irdyWuBeyWAp21cj
;; hex private key f6588520e266c8ec43672fc97aa23a173831cd89be50823c7dca629b566d26b3
;; hex public key 030046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563
;; address bcrt1q5s4azffap92uc3qvujetg9ksgja424ef2hrsr5
;; address hash160 a42bd1253d0955cc440ce4b2b416d044bb555729

;; mock funding tx:
;; 5 BTC to bcrt1pagnw9svfx6hsamy8nulqnrzf4aumdta7jquc0ctzf0wtwlw3709sau2mh2
;; 0200000000010127698312f68d100f849f3bcf550197cf98bdfcbd2cf232dc3d851db63e791e940000000000fdffffff020065cd1d00000000225120ea26e2c18936af0eec879f3e098c49af79b6afbe903987e1624bdcb77dd1f3cbf58c380c010000002251202e4c5cf74b0b9cf504d4e6d71cb1216d6556acd5f225224a5515607134b45ee002473044022052f9429e107630261a2c6bb8681f21fe24efe4aa0e496fb82891db4826b7a654022037d197d46cddc63a6c0b1f3b4579732eb87b503b9013bc6a724bafe728f9799a0121026799af6b47e0cdfebd3acea463dce2cfb6e65bcc477c1fff907ba48827226d1ccd000000

;; Address: bcrt1plp2u5s6q97ueehsw8e35k96kgftplavylemdz6rxxrv06t4f8vmq33c9jr
;; 50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0 1 '[OP_TRUE]'

(define-constant version-P2TR 0x06)

(define-constant mock-peg-wallet { version: version-P2TR, hashbytes: 0xf855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36 })
(define-constant mock-peg-cycle u0)
(define-constant mock-burnchain-height u3)

;;(define-constant mock-escrow-pubkey-1 0x6a30ab928118563dc27888d9af98d0138c32a8ed0efc9dcd0bf4cc4b503114de)
(define-constant mock-escrow-address-1 "bcrt1pdgc2hy5prptrmsnc3rv6lxxszwxr928dpm7fmngt7nxyk5p3zn0qphrsks")

(define-constant mock-unlock-script-1 0x183c001a7321b74e2b6a7e949e6c4ad313035b1665095017007520f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36ac)

(define-constant mock-tx-1 0x020000000001010052458c56fea00527237f73d6b7bb4cbaf1f5436c9d2673ae2e0164f4ad17d20000000000fdffffff010065cd1d00000000225120f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b360340000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f3c183c001a7321b74e2b6a7e949e6c4ad313035b1665095017007520f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36ac41c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac074708f439116be919de13c6d3200d2305fcbdf5a9e7d2c079e85b427bb110e9000000000)
(define-constant mock-wtxid-1 0x13d6ccd90dc236915d16dabe29fc02c00d4f5aad35577b43358a233d6e4620fd)
(define-constant mock-txid-1 0xcd2662154e6d76b2b2b92e70c0cac3ccf534f9b74eb5b89819ec509083d00a50)

(define-constant mock-witness-index-1 u0)

(define-constant mock-wtxid-1-le 0xfd20466e3d238a35437b5735ad5a4f0dc002fc29beda165d9136c20dd9ccd613)
(define-constant mock-txid-1-le 0xcf21d79d8a8104f1f50473bb0a1bbc20e30dbc1eba2be0ef66478adb41ee6801)

(define-constant mock-value-tx-1 u500000000)

(define-constant mock-witness-root-hash-1-le 0x5c44856d25f0c9c3149dabb4efba3b9ddec1f8f833921dd323bd6d1ac1bd277f)

(define-constant mock-coinbase-witness-reserved-data 0x0000000000000000000000000000000000000000000000000000000000000000)

(define-constant mock-coinbase-tx-1 0x01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff1e0383a02519444d47426c6f636b636861696e309e3c092400000000000000ffffffff029e49250000000000160014b23716e183ba0949c55d6cac21a3e94176eed1120000000000000000266a24aa21a9ed8a3bb68aa55850328ea8233754a147464b8580c15460c4ffb928ab23cf0d198b0120000000000000000000000000000000000000000000000000000000000000000000000000)
(define-constant mock-coinbase-wtxid-1 0x0000000000000000000000000000000000000000000000000000000000000000)

(define-constant mock-block-header-1 0x0000000000000000000000000000000000000000000000000000000000000000000000000b5c59d28b48942bba392cabfc2459c2842e6549e85bc2714b0ebce5c1c925d7000000000000000000000000)
(define-constant mock-block-header-hash-1-be 0x6f028d4c95181966e53930fb034301081aaefed6f043ea1d86b329274a354b92)

(define-read-only (get-sbtc-balance (who principal))
	(unwrap! (contract-call? .sbtc-token get-balance who) u0)
)

(define-public (prepare-add-test-to-protocol)
	(contract-call? .sbtc-testnet-debug-controller set-protocol-contract (as-contract tx-sender) true)
)

(define-public (prepare)
	(begin
		;; Add the test contract to the protocol contract set.
		(try! (prepare-add-test-to-protocol))
		;; Add mock peg wallet adress to registry for test cycle
		(try! (contract-call? .sbtc-registry insert-cycle-peg-wallet mock-peg-cycle mock-peg-wallet))
		;; Mine a fake burnchain block that includes mock transactions
		(try! (contract-call? .sbtc-testnet-debug-controller simulate-mine-solo-burnchain-block mock-burnchain-height (list mock-tx-1)))
		(unwrap! (contract-call? .clarity-bitcoin mock-add-burnchain-block-header-hash mock-burnchain-height mock-block-header-hash-1-be) (err u112233))
		(ok true)
	)
)

(define-public (test-extract-recipient-and-spending-pubkey)
	(let (
			(script 0x183c001a7321b74e2b6a7e949e6c4ad313035b16650950170075200046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563ac)
			(expected (ok { input-spending-pubkey: 0x0046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563,
				recipient: 'ST1SJ3DTE5DN7X54YDH5D64R3BCB6A2AG2ZQ8YPD5 }))
			(actual (contract-call? .sbtc-peg-in-processor verify-extract-unlock-script script))
		)
		(asserts! (is-eq actual expected) (err {expected: expected, actual: actual, msg: "Result mismatch"}))
		(ok true)
	)
)

(define-public (test-extract-recipient-and-spending-pubkey-invalid-opcode)
	(let (
			;; FF in place of sbtc opcode
			(script 0x18ff001a7321b74e2b6a7e949e6c4ad313035b16650950170075200046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563ac)
			(expected err-script-invalid-opcode)
			(result (contract-call? .sbtc-peg-in-processor verify-extract-unlock-script script))
		)
		(asserts! (is-eq result expected) (err {expected: (some expected), actual: (some result), msg: "Result mismatch"}))
		(ok true)
	)
)

(define-public (test-extract-recipient-and-spending-pubkey-invalid-version)
	(let (
			;; FF in place of payload version
			(script 0x183cff1a7321b74e2b6a7e949e6c4ad313035b16650950170075200046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563ac)
			(expected err-script-invalid-version)
			(result (contract-call? .sbtc-peg-in-processor verify-extract-unlock-script script))
		)
		(asserts! (is-eq result expected) (err {expected: (some expected), actual: (some result), msg: "Result mismatch"}))
		(ok true)
	)
)

(define-public (test-extract-recipient-and-spending-pubkey-not-op-drop)
	(let (
			;; FF in place of OP_DROP
			(script 0x183c001a7321b74e2b6a7e949e6c4ad313035b166509501700ff200046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563ac)
			(expected err-script-not-op-drop)
			(result (contract-call? .sbtc-peg-in-processor verify-extract-unlock-script script))
		)
		(asserts! (is-eq result expected) (err {expected: (some expected), actual: (some result), msg: "Result mismatch"}))
		(ok true)
	)
)

(define-public (test-extract-recipient-and-spending-pubkey-checksig-missing)
	(let (
			;; removed OP_CHECKSIG at the end (0xac)
			(script 0x183c001a7321b74e2b6a7e949e6c4ad313035b16650950170075200046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563)
			(expected err-script-checksig-missing)
			(result (contract-call? .sbtc-peg-in-processor verify-extract-unlock-script script))
		)
		(asserts! (is-eq result expected) (err {expected: (some expected), actual: (some result), msg: "Result mismatch"}))
		(ok true)
	)
)

(define-public (test-extract-recipient-and-spending-pubkey-invalid-contract-principal)
	(let (
			;; Invalid contract name (0xff)
			(script 0x193c001a7321b74e2b6a7e949e6c4ad313035b166509501701ff75200046422d30ec92c568e21be4b9579cfed8e71ba0702122b014755ae0e23e3563ac)
			(expected err-script-invalid-principal)
			(result (contract-call? .sbtc-peg-in-processor verify-extract-unlock-script script))
		)
		(asserts! (is-eq result expected) (err {expected: (some expected), actual: (some result), msg: "Result mismatch"}))
		(ok true)
	)
)

;; @name Test peg-in reveal proof (mints sBTC)
;; @mine-blocks-before 5
(define-public (test-peg-in-reveal)
	(let (
    (result
      (contract-call? .sbtc-peg-in-processor complete-peg-in
			mock-peg-cycle
			mock-burnchain-height ;; burn-height
			mock-tx-1 ;; tx
			mock-block-header-1 ;; header
			u1 ;; tx-index
			u1 ;; tree-depth
			(list mock-coinbase-wtxid-1) ;; wproof
			mock-witness-root-hash-1-le
			mock-coinbase-witness-reserved-data
			mock-witness-index-1
			mock-coinbase-tx-1 ;; ctx
			(list mock-txid-1) ;; cproof
			)
      )
    )
		(unwrap! result (err {msg: "Expected ok, got err", actual: (some result)}))
		(asserts! (is-eq (get-sbtc-balance wallet-1) mock-value-tx-1) (err {msg: "User did not receive the expected sBTC", actual: none}))
		(ok true)
	)
)

;; @name Cannot submit the same proof twice
;; @mine-blocks-before 5
(define-public (test-peg-in-reveal-no-repeat)
	(let ((result (contract-call? .sbtc-peg-in-processor complete-peg-in
			mock-peg-cycle
			mock-burnchain-height ;; burn-height
			mock-tx-1 ;; tx
			mock-block-header-1 ;; header
			u1 ;; tx-index
			u1 ;; tree-depth
			(list mock-coinbase-wtxid-1) ;; wproof
			mock-witness-root-hash-1-le
			mock-coinbase-witness-reserved-data
			mock-witness-index-1
			mock-coinbase-tx-1 ;; ctx
			(list mock-txid-1) ;; cproof
			))
		(result2 (contract-call? .sbtc-peg-in-processor complete-peg-in
			mock-peg-cycle
			mock-burnchain-height ;; burn-height
			mock-tx-1 ;; tx
			mock-block-header-1 ;; header
			u1 ;; tx-index
			u1 ;; tree-depth
			(list mock-coinbase-wtxid-1) ;; wproof
			mock-witness-root-hash-1-le
			mock-coinbase-witness-reserved-data
			mock-witness-index-1
			mock-coinbase-tx-1 ;; ctx
			(list mock-txid-1) ;; cproof
			))
      )
		(unwrap! result (err {msg: "Expected ok, got err", actual: (some result)}))
		(asserts! (is-eq (get-sbtc-balance wallet-1) mock-value-tx-1) (err {msg: "User did not receive the expected sBTC", actual: none}))
		(asserts! (is-eq result2 err-burn-tx-already-processed) (err {msg: "Second call should have failed with err-burn-tx-already-processed", actual: (some result2)}))
		(ok true)
	)
)

