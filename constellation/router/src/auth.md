
// when user 
[constellation/router/src/test/test.rs:376:5] test.env.auths() = [
    (
        Contract(CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHK3M),
        AuthorizedInvocation {
            function: Contract(
                (
                    Contract(CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARQG5),
                    Symbol(mint_exact_constellation),
                    Vec(Ok(I128(1000000)), Ok(I128(10)), Ok(Address(obj#4571)), Ok(Address(obj#4575)), Ok(Address(obj#4579)), Ok(U64(10000000))),
                ),
            ),
            sub_invocations: [
                AuthorizedInvocation {
                    function: Contract(
                        (
                            Contract(CBRIAA73VOIKPZYM5G3LGPF3NGCFXLR3IW22MKEYJAB3QBOMTUTRCASK),
                            Symbol(swap_tokens_for_exact_tokens),
                            Vec(Ok(I128(10032)), Ok(I128(1000000)), Ok(Vec(obj#4599)), Ok(Address(obj#4603)), Ok(U64(10000000))),
                        ),
                    ),
                    sub_invocations: [
                        AuthorizedInvocation {
                            function: Contract(
                                (
                                    Contract(CCVQTUQIJR624NNEI5TORM2BHEXTSDMY5ZB3CYJKAATGJQCY7LU2MD45),
                                    Symbol(transfer),
                                    Vec(Ok(Address(obj#4615)), Ok(Address(obj#4619)), Ok(I128(10064))),
                                ),
                            ),
                            sub_invocations: [],
                        },
                    ],
                },
            ],
        },
    ),
]



// when e.current_address()

 test.env.auths() = [
    (
        Contract(CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHK3M),
        AuthorizedInvocation {
            function: Contract(
                (
                    Contract(CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARQG5),
                    Symbol(mint_exact_constellation),
                    Vec(Ok(I128(1000000)), Ok(I128(10)), Ok(Address(obj#4559)), Ok(Address(obj#4563)), Ok(Address(obj#4567)), Ok(U64(10000000))),
                ),
            ),
            sub_invocations: [],
        },
    ),
    (
        Contract(CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAARQG5),
        AuthorizedInvocation {
            function: Contract(
                (
                    Contract(CCVQTUQIJR624NNEI5TORM2BHEXTSDMY5ZB3CYJKAATGJQCY7LU2MD45),
                    Symbol(transfer),
                    Vec(Ok(Address(obj#4581)), Ok(Address(obj#4585)), Ok(I128(10064))),
                ),
            ),
            sub_invocations: [],
        },
    ),
]