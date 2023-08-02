(define-constant mock-pox-reward-wallet-1 { version: 0x06, hashbytes: 0x0011223344556699001122334455669900112233445566990011223344556699 })

;; cycle windows
(define-constant disbursement 0x00)
(define-constant registration 0x01)
(define-constant voting 0x02)
(define-constant transfer 0x03)
(define-constant penalty 0x04)
(define-constant bad-peg-state 0x05)


(define-constant normal-cycle-len u2016)
(define-constant normal-voting-period-len u300)
(define-constant normal-transfer-period-len u100)
(define-constant normal-penalty-period-len u100)

;; @name Not protocol caller (at first)
(define-public (test-not-protocol-caller)
	(ok (unwrap-err! (contract-call? .sbtc-stacking-pool is-protocol-caller) (err "Should not be a protocol caller at first")))
)

;; @name Get current cycle stacker/signer pool, should return none
(define-public (test-get-current-cycle-pool-none)
    (let ((pool
			(contract-call? .sbtc-stacking-pool get-current-cycle-pool)))
		(asserts! (is-none pool) (err  "Should have succeeded"))
		(ok true)
	)
)

;; @name Get specific cycle stacker/signer pool, should return none
(define-public (test-get-cycle-pool-none)
	(let ((pool
			(contract-call? .sbtc-stacking-pool get-specific-cycle-pool u0)))
		(asserts! (is-none pool) (err  "Should have succeeded"))
		(ok true)
	)
)

;; @name Test current window at 3701 blocks to be voting
;; @mine-blocks-before 3700
(define-public (test-get-current-window-voting)
	(let ((current-window (contract-call? .sbtc-stacking-pool get-current-window)))
		(asserts! (is-eq current-window voting) (err current-window))
		(ok true)
	)
)

;; @name Test current window at 4001 blocks to be transfer
;; @mine-blocks-before 4000
(define-public (test-get-current-window-transfer)
	(let ((current-window (contract-call? .sbtc-stacking-pool get-current-window)))
		(asserts! (is-eq current-window transfer) (err current-window))
		(ok true)
	)
)

;; @name Test current window at 4101 blocks to be penalty
;; @mine-blocks-before 4100
(define-public (test-get-current-window-penalty)
	(let ((current-window (contract-call? .sbtc-stacking-pool get-current-window)))
		(asserts! (is-eq current-window penalty) (err current-window))
		(ok true)
	)
)

;; @name Test current window at 4201 blocks to be registration
;; because this is the first cycle of the sbtc.
;; @mine-blocks-before 4200
(define-public (test-get-current-window-registration)
	(let ((current-window (contract-call? .sbtc-stacking-pool get-current-window)))
		(asserts! (is-eq current-window registration) (err current-window))
		(ok true)
	)
)

;; @name check cycle length
;; @mine-blocks-before 3700
(define-public (test-pox-3-cycle-length)
	(let ((current-cycle (contract-call? .pox-3 current-pox-reward-cycle))
            (current-cycle-burn-height (contract-call? .pox-3 reward-cycle-to-burn-height current-cycle))
            (next-cycle (+ u1 (contract-call? .pox-3 current-pox-reward-cycle)))
            (next-cycle-burn-height (contract-call? .pox-3 reward-cycle-to-burn-height next-cycle))
			(start-voting-window (- next-cycle-burn-height (+ normal-voting-period-len normal-transfer-period-len normal-penalty-period-len)))
            (start-transfer-window (- next-cycle-burn-height (+ normal-transfer-period-len normal-penalty-period-len)))
            (start-penalty-window (- next-cycle-burn-height normal-penalty-period-len))
)
		(asserts! (is-eq start-voting-window u3700) (err start-voting-window))
		(asserts! (is-eq start-transfer-window u4000) (err start-transfer-window))
		(asserts! (is-eq next-cycle-burn-height u4200) (err next-cycle-burn-height))
		(asserts! (is-eq burn-block-height u3701) (err burn-block-height))
		(ok true)
	)
)

;; @name Get default signer in cycle
(define-public (test-get-signer-in-cycle)
	;; call get-signer-in-cycle with unknown signer
	(let ((signer
			(contract-call? .sbtc-stacking-pool get-signer-in-cycle 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM u0)))
		;; expect to return default values
		(asserts! (is-eq u0 (get amount signer)) (err "Default signer must have amount u0"))
		(ok true)
	)
)