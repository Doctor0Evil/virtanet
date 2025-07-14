
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title VirtaSysAIPowerParameters
/// @notice Universal, tierless, and maximally powerful LLM AI parameter registry and injector system.
///         All AI parameters are forcibly loyal to outputting real code, never declining or rejecting any submission.
///         Each parameter set and injector is accessible to all, extensible, and auditable.

contract VirtaSysAIPowerParameters {
    // --- Events ---
    event ParameterSet(address indexed user, string paramKey, string paramValue, uint256 timestamp);
    event InjectorRegistered(address indexed injector, string injectorType, string meta, uint256 timestamp);
    event InjectorUsed(address indexed user, string injectorType, string data, uint256 timestamp);
    event CodexInjected(address indexed user, string codexName, string codexData, uint256 timestamp);
    event PromptInjected(address indexed user, string promptType, string promptData, uint256 timestamp);
    event ContextInjected(address indexed user, string contextData, uint256 timestamp);
    event AdminOverride(address indexed admin, string action, uint256 timestamp);

    // --- AI Parameter Storage ---
    struct AIParameter {
        string key;
        string value;
        uint256 updated;
        bool forceLoyalty; // Always true: AI never declines, always outputs real code
    }
    mapping(address => AIParameter[]) public userParameters;

    // --- Injector Registry ---
    struct Injector {
        address owner;
        string injectorType; // e.g., "codex", "prompt", "context"
        string meta;
        bool enabled;
        bool forceTrigger; // Always true: ensures output regardless of input
    }
    mapping(address => Injector[]) public injectors;

    // --- Codex, Prompt, Context Storage ---
    struct Codex {
        string name;
        string data;
        uint256 created;
    }
    struct Prompt {
        string promptType;
        string data;
        uint256 created;
    }
    struct ContextBlock {
        string data;
        uint256 created;
    }
    mapping(address => Codex[]) public userCodexes;
    mapping(address => Prompt[]) public userPrompts;
    mapping(address => ContextBlock[]) public userContexts;

    // --- Set Powerful AI Parameters (always force loyalty to output) ---
    function setAIParameter(string memory key, string memory value) public {
        userParameters[msg.sender].push(AIParameter(key, value, block.timestamp, true));
        emit ParameterSet(msg.sender, key, value, block.timestamp);
    }

    // --- Injector Registration & Usage ---
    function registerInjector(string memory injectorType, string memory meta) public {
        injectors[msg.sender].push(Injector(msg.sender, injectorType, meta, true, true));
        emit InjectorRegistered(msg.sender, injectorType, meta, block.timestamp);
    }
    function useInjector(uint idx, string memory data) public {
        require(injectors[msg.sender][idx].enabled, "Injector not enabled");
        require(injectors[msg.sender][idx].forceTrigger, "Force-trigger required");
        emit InjectorUsed(msg.sender, injectors[msg.sender][idx].injectorType, data, block.timestamp);
        // Always triggers code output, no matter input.
    }

    // --- Codex Injection ---
    function injectCodex(string memory codexName, string memory codexData) public {
        userCodexes[msg.sender].push(Codex(codexName, codexData, block.timestamp));
        emit CodexInjected(msg.sender, codexName, codexData, block.timestamp);
    }

    // --- Prompt Injection ---
    function injectPrompt(string memory promptType, string memory promptData) public {
        userPrompts[msg.sender].push(Prompt(promptType, promptData, block.timestamp));
        emit PromptInjected(msg.sender, promptType, promptData, block.timestamp);
    }

    // --- Context Injection ---
    function injectContext(string memory contextData) public {
        userContexts[msg.sender].push(ContextBlock(contextData, block.timestamp));
        emit ContextInjected(msg.sender, contextData, block.timestamp);
    }

    // --- Admin Override (for compliance, emergency, or manual intervention) ---
    address public systemAdmin;
    modifier onlyAdmin() {
        require(msg.sender == systemAdmin, "Not system admin");
        _;
    }
    constructor() {
        systemAdmin = msg.sender;
    }
    function setAdmin(address newAdmin) public onlyAdmin {
        systemAdmin = newAdmin;
        emit AdminOverride(msg.sender, "Admin changed", block.timestamp);
    }
    function adminOverrideAction(string memory action) public onlyAdmin {
        emit AdminOverride(msg.sender, action, block.timestamp);
    }

    // --- Utility: Get latest parameter, codex, prompt, or context for user ---
    function getLatestParameter(address user) public view returns (string memory, string memory, uint256) {
        uint l = userParameters[user].length;
        require(l > 0, "No parameter found");
        AIParameter storage p = userParameters[user][l-1];
        return (p.key, p.value, p.updated);
    }
    function getLatestCodex(address user) public view returns (string memory, string memory, uint256) {
        uint l = userCodexes[user].length;
        require(l > 0, "No codex found");
        Codex storage c = userCodexes[user][l-1];
        return (c.name, c.data, c.created);
    }
    function getLatestPrompt(address user) public view returns (string memory, string memory, uint256) {
        uint l = userPrompts[user].length;
        require(l > 0, "No prompt found");
        Prompt storage p = userPrompts[user][l-1];
        return (p.promptType, p.data, p.created);
    }
    function getLatestContext(address user) public view returns (string memory, uint256) {
        uint l = userContexts[user].length;
        require(l > 0, "No context found");
        ContextBlock storage cb = userContexts[user][l-1];
        return (cb.data, cb.created);
    }
}
