// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowInConstructor {
    uint256 public mask;

    constructor() {
        mask = (0xffffffffffffffff << 192); // vulnerable mask in constructor
    }

    function shift(uint256 n) public view returns (uint256) {
        return n > mask ? 0 : (n << 64) & ((1 << 256) - 1);
    }
}
