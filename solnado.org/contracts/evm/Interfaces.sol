// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface ISheriffMixer {
    function depositAndMix(address token, uint256 amount) external;
    function processMixing(address user, address token) external;
    function setCleanWallets(address[] calldata _cleanWallets) external;
}
