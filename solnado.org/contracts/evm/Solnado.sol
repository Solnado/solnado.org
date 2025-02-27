// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./Interfaces.sol";
import "./Utils.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Solnado.org is Ownable {
    using Utils for uint256;

    uint256 public constant FEE_PERCENT = 7; // %0.7 Mixing Ücreti (1000 birim = %100)
    address public immutable SHERIFF_TOKEN;
    address[] private cleanWallets;
    mapping(address => uint256) public depositTimestamps;

    event Deposited(address indexed sender, uint256 amount, address token);
    event Mixed(address indexed recipient, uint256 amount, address token);
    
    constructor(address _sheriffToken, address[] memory _cleanWallets) {
        SHERIFF_TOKEN = _sheriffToken;
        cleanWallets = _cleanWallets;
    }

    function depositAndMix(address token, uint256 amount) external {
        require(amount > 0, "Amount must be greater than 0");

        IERC20(token).transferFrom(msg.sender, address(this), amount);
        depositTimestamps[msg.sender] = block.timestamp;

        emit Deposited(msg.sender, amount, token);
    }

    function processMixing(address user, address token) external onlyOwner {
        require(depositTimestamps[user] > 0, "No deposit found");
        
        uint256 balance = IERC20(token).balanceOf(address(this));
        uint256 fee = balance * FEE_PERCENT / 1000;
        uint256 mixAmount = balance - fee;

        address finalWallet = Utils.selectRandomWallet(cleanWallets);
        IERC20(token).transfer(finalWallet, mixAmount);

        delete depositTimestamps[user];

        emit Mixed(user, mixAmount, token);
    }

    function setCleanWallets(address[] calldata _cleanWallets) external onlyOwner {
        cleanWallets = _cleanWallets;
    }
}
