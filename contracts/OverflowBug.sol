// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowBug {
    uint256 constant MASK = (0xffffffffffffffff << 192); // vulnerable pattern

    function buggyShift(uint256 n) public pure returns (uint256) {
        if (n > MASK) {
            return 0;
        } else {
            return (n << 64) & ((1 << 256) - 1);
        }
    }
}
