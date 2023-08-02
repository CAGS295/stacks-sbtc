(define-constant version-P2WPKH 0x04)
(define-constant version-P2TR 0x06)
(define-constant version-invalid 0x99)

;; @name Extract protocol witness script for peg-out reveal transactions.
(define-public (test-find-peg-out-reveal-protocol-unlock-witness)
	(let 
		(
			(expected (some 0x183c001a7321b74e2b6a7e949e6c4ad313035b1665095017007520f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36ac))
			(actual (contract-call? .sbtc-btc-tx-helper find-protocol-unlock-witness (list 0x000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f 0x183c001a7321b74e2b6a7e949e6c4ad313035b1665095017007520f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36ac 0xc050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac074708f439116be919de13c6d3200d2305fcbdf5a9e7d2c079e85b427bb110e90)))
		)
		(ok (asserts! (is-eq expected actual) (err {expected:expected, actual:actual, msg:"Could not find the witness (should have returned the second item)"})))
	)
)

;; @name Convert hashbytes to P2TR scriptPubkey
(define-public (test-hashbytes-to-scriptpubkey)
	(let
		(
			(expected (some 0x5120f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36))
			(actual (contract-call? .sbtc-btc-tx-helper hashbytes-to-scriptpubkey { version: version-P2TR, hashbytes: 0xf855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36 }))
		) 
		(ok (asserts! (is-eq expected actual) (err {expected: expected, actual:actual, msg:"Mismatch"})))
	)
)


;; @name Convert hashbytes to scriptPubkey with non-p2tr version
(define-public (test-hashbytes-to-scriptpubkey-with-P2WPKH-version)
	(let ((expected (some 0x0020f855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36))
		(actual (contract-call? .sbtc-btc-tx-helper hashbytes-to-scriptpubkey { version: version-P2WPKH, hashbytes: 0xf855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36 })))
		(asserts! (is-eq actual expected)
			(err {actual: actual, expected: expected}))
		(ok true))
)

;; @name test error handling for hashbytes to scriptPubkey with invalid version
(define-public (test-hashbytes-to-scriptpubkey-with-invalid-version)
	(let ((actual (contract-call? .sbtc-btc-tx-helper hashbytes-to-scriptpubkey { version: version-invalid, hashbytes: 0xf855ca43402fb99cde0e3e634b175642561ff584fe76d1686630d8fd2ea93b36 })))

		(asserts! (is-eq actual none)
			(err {actual: actual, expected: none}))
		(ok true))
)
