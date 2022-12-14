////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    pair
    (
        addAdmin
        addInitialLiquidity
        addLiquidity
        addToPauseWhitelist
        addTrustedSwapPair
        getAmountIn
        getAmountOut
        getBPAddConfig
        getBPRemoveConfig
        getBPSwapConfig
        getEquivalent
        getExternSwapGasLimit
        getFeeDestinations
        getFeeState
        getFeesCollectorAddress
        getFeesCollectorCutPercentage
        getFirstTokenId
        getInitialLiquidtyAdder
        getLockingDeadlineEpoch
        getLockingScAddress
        getLpTokenIdentifier
        getNumAddsByAddress
        getNumRemovesByAddress
        getNumSwapsByAddress
        getPermissions
        getReserve
        getReservesAndTotalSupply
        getRouterManagedAddress
        getRouterOwnerManagedAddress
        getSecondTokenId
        getSpecialFee
        getState
        getTokensForGivenPosition
        getTotalFeePercent
        getTotalSupply
        getTrustedSwapPairs
        getUnlockEpoch
        getWhitelistedManagedAddresses
        pause
        removeAdmin
        removeFromPauseWhitelist
        removeLiquidity
        removeLiquidityAndBuyBackAndBurnToken
        removeTrustedSwapPair
        removeWhitelist
        resume
        setBPAddConfig
        setBPRemoveConfig
        setBPSwapConfig
        setExternSwapGasLimit
        setFeeOn
        setFeePercents
        setLockingDeadlineEpoch
        setLockingScAddress
        setLpTokenIdentifier
        setMaxObservationsPerRecord
        setStateActiveNoSwaps
        setUnlockEpoch
        setupFeesCollector
        swapNoFeeAndForward
        swapTokensFixedInput
        swapTokensFixedOutput
        updateAndGetSafePrice
        updateAndGetTokensForGivenPositionWithSafePrice
        whitelist
    )
}

elrond_wasm_node::wasm_empty_callback! {}
