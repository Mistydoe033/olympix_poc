// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OverflowWithEvent {
    uint256 public balance;

    event BalanceUpdated(uint256 oldBalance, uint256 newBalance);
    event OverflowDetected(uint256 value, string operation);

    function addBalance(uint256 amount) public {
        uint256 oldBalance = balance;
        balance = balance + amount; // Potential overflow
        emit BalanceUpdated(oldBalance, balance);

        if (balance < oldBalance) {
            emit OverflowDetected(amount, "addition");
        }
    }

    function subtractBalance(uint256 amount) public {
        uint256 oldBalance = balance;
        balance = balance - amount; // Potential underflow
        emit BalanceUpdated(oldBalance, balance);

        if (balance > oldBalance) {
            emit OverflowDetected(amount, "subtraction");
        }
    }
}
