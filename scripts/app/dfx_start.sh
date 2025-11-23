dfx stop

dfx start --clean --system-canisters |& \
    perl -pe 's/^\d{4}-\d{2}-\d{2} //; s/\.(\d{3})\d*/\.$1/; s/\[Canister ([^]]+)\]/[\1]/; s/ UTC://'
