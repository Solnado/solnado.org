// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

library Utils {
    function random(uint256 seed) internal view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(block.timestamp, msg.sender, seed))) % 100;
    }

    function selectRandomWallet(address[] memory wallets) internal view returns (address) {
        require(wallets.length > 0, "No clean wallets available");
        uint256 index = random(wallets.length);
        return wallets[index];
    }
}
