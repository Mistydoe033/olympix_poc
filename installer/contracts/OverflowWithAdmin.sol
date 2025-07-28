// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowWithAdmin {
    address public admin;
    uint256 constant MASK = (0xffffffffffffffff << 192);

    constructor() {
        admin = msg.sender;
    }

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    function adminShift(uint256 n) public onlyAdmin returns (uint256) {
        return n > MASK ? 0 : (n << 64) & ((1 << 256) - 1);
    }
}
