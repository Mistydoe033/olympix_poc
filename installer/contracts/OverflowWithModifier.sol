// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowWithModifier {
    uint256 constant MASK = (0xffffffffffffffff << 192);

    modifier checkInput(uint256 n) {
        require(n < (MASK << 1), "Too large");
        _;
    }

    function shiftWithModifier(
        uint256 n
    ) public pure checkInput(n) returns (uint256) {
        return (n << 64) & ((1 << 256) - 1);
    }
}
