// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowWithFallback {
    uint256 constant MASK = (0xffffffffffffffff << 192);
    uint256 public lastResult;

    fallback() external payable {
        uint256 n = msg.value;
        lastResult = n > MASK ? 0 : (n << 64) & ((1 << 256) - 1);
    }
}
