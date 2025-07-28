// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

library OverflowLib {
    function buggyShift(uint256 n) internal pure returns (uint256) {
        uint256 mask = (0xffffffffffffffff << 192);
        return n > mask ? 0 : (n << 64) & ((1 << 256) - 1);
    }
}

contract OverflowWithLibrary {
    using OverflowLib for uint256;

    function doShift(uint256 n) public pure returns (uint256) {
        return n.buggyShift();
    }
}
