// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowWithPayable {
    uint256 constant MASK = (0xffffffffffffffff << 192);
    event Paid(address indexed from, uint256 amount, uint256 result);

    function payAndShift() public payable {
        uint256 result = msg.value > MASK
            ? 0
            : (msg.value << 64) & ((1 << 256) - 1);
        emit Paid(msg.sender, msg.value, result);
    }
}
