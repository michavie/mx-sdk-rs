////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    use_module
    (
        callBack
        addAdmin
        addMergeableTokensToWhitelist
        call_derived_not_admin_only
        call_derived_not_owner_only
        call_mod_a
        call_mod_b
        call_mod_c
        cancel
        changeLockTimeAfterVotingEndsInBlocks
        changeMinTokenBalanceForProposing
        changeQuorum
        changeVotingDelayInBlocks
        changeVotingPeriodInBlocks
        checkFeatureGuard
        checkPause
        claimDeveloperRewards
        countTo100
        depositTokensForAction
        dnsRegister
        downvote
        execute
        getAdmins
        getGovernanceTokenId
        getLockTimeAfterVotingEndsInBlocks
        getMergeableTokensWhitelist
        getMergedTokenId
        getMinTokenBalanceForProposing
        getProposalActions
        getProposalDescription
        getProposalStatus
        getProposer
        getQuorum
        getTotalDownvotes
        getTotalVotes
        getVotingDelayInBlocks
        getVotingPeriodInBlocks
        isAdmin
        isPaused
        issueMergedToken
        issueToken
        mergeTokens
        mergeTokensCustomAttributes
        only_admin_mod_endpoint
        only_owner_mod_endpoint
        pause
        propose
        queue
        removeAdmin
        removeMergeableTokensFromWhitelist
        setFeatureFlag
        slashMember
        splitTokenPartial
        splitTokens
        stake
        unpause
        unstake
        vote
        voteSlashMember
        withdrawGovernanceTokens
    )
}
