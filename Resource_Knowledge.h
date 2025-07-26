#ifndef APOCALITTZ_RESOURCE_KNOWLEDGE_H
#define APOCALITTZ_RESOURCE_KNOWLEDGE_H

#include <string>
#include <vector>
#include <map>
#include <memory>
#include <functional>
#include <fstream>
#include <random>
#include <ctime>
#include <iostream>
#include <sqlite3.h>
#include <curl/curl.h>

namespace Apocalittz {

class MenuItem {
public:
    std::string name;
    std::function<void()> action;
    bool enabled;

    MenuItem(const std::string& n, std::function<void()> act, bool en = true)
        : name(n), action(act), enabled(en) {}
};

class ResourceCache {
private:
    std::map<std::string, std::string> cachedResources;
    std::map<std::string, time_t> cacheTimestamps;
    const int CACHE_TTL = 3600;

public:
    void cacheResource(const std::string& resourceId, const std::string& data) {
        cachedResources[resourceId] = data;
        cacheTimestamps[resourceId] = time(nullptr);
    }

    bool isResourceValid(const std::string& resourceId) const {
        auto it = cacheTimestamps.find(resourceId);
        if (it == cacheTimestamps.end()) return false;
        return (time(nullptr) - it->second) < CACHE_TTL;
    }

    std::string getResource(const std::string& resourceId) const {
        auto it = cachedResources.find(resourceId);
        if (it != cachedResources.end() && isResourceValid(resourceId)) {
            return it->second;
        }
        return "";
    }

    void clearExpired() {
        for (auto it = cacheTimestamps.begin(); it != cacheTimestamps.end();) {
            if (!isResourceValid(it->first)) {
                cachedResources.erase(it->first);
                it = cacheTimestamps.erase(it);
            } else {
                ++it;
            }
        }
    }

    void clearAll() {
        cachedResources.clear();
        cacheTimestamps.clear();
    }
};

class KnowledgePool {
private:
    std::map<std::string, std::vector<std::pair<std::string, double>>> knowledgeBase;
    std::string apiEndpoint;
    std::string apiKey;

    static size_t WriteCallback(void* contents, size_t size, size_t nmemb, std::string* userp) {
        userp->append((char*)contents, size * nmemb);
        return size * nmemb;
    }

    void fetchFromApi(const std::string& query) {
        CURL* curl = curl_easy_init();
        if (!curl) {
            std::cerr << "CURL initialization failed.\n";
            knowledgeBase[query].push_back({"Fallback fact about " + query, 1.0});
            return;
        }

        std::string response;
        std::string payload = R"({"model":"grok-beta","messages":[{"role":"user","content":"Provide facts about )" + query + R"("}],"max_tokens":100,"temperature":0.2})";
        struct curl_slist* headers = nullptr;
        headers = curl_slist_append(headers, ("Authorization: Bearer " + apiKey).c_str());
        headers = curl_slist_append(headers, "Content-Type: application/json");

        curl_easy_setopt(curl, CURLOPT_URL, apiEndpoint.c_str());
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, payload.c_str());
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

        CURLcode res = curl_easy_perform(curl);
        if (res != CURLE_OK) {
            std::cerr << "API Error: " << curl_easy_strerror(res) << "\n";
            knowledgeBase[query].push_back({"Fallback fact about " + query, 1.0});
        } else {
            std::vector<std::string> facts;
            size_t pos = 0;
            while ((pos = response.find('\n')) != std::string::npos) {
                facts.push_back(response.substr(0, pos));
                response.erase(0, pos + 1);
            }
            if (!response.empty()) facts.push_back(response);
            for (const auto& fact : facts) {
                if (!fact.empty()) {
                    knowledgeBase[query].push_back({fact, 1.0 / (facts.size() + 1)});
                }
            }
            if (facts.empty()) {
                knowledgeBase[query].push_back({"Fallback fact about " + query, 1.0});
            }
        }

        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
    }

public:
    KnowledgePool(const std::string& endpoint = "https://api.x.ai/v1/chat/completions",
                  const std::string& key = std::getenv("XAI_API_KEY") ? std::getenv("XAI_API_KEY") : "")
        : apiEndpoint(endpoint), apiKey(key) {
        curl_global_init(CURL_GLOBAL_ALL);
        if (apiKey.empty()) {
            std::cerr << "Warning: XAI_API_KEY not set.\n";
        }
    }

    ~KnowledgePool() {
        curl_global_cleanup();
    }

    void addKnowledge(const std::string& topic, const std::string& fact, double weight = 1.0) {
        knowledgeBase[topic].push_back({fact, weight});
        normalizeWeights(topic);
    }

    void normalizeWeights(const std::string& topic) {
        double total = 0.0;
        for (const auto& [fact, weight] : knowledgeBase[topic]) {
            total += weight;
        }
        if (total > 0) {
            for (auto& [fact, weight] : knowledgeBase[topic]) {
                weight /= total;
            }
        }
    }

    std::string getKnowledge(const std::string& topic) {
        auto it = knowledgeBase.find(topic);
        if (it != knowledgeBase.end() && !it->second.empty()) {
            std::random_device rd;
            std::mt19937 gen(rd());
            std::vector<double> weights;
            for (const auto& [fact, weight] : it->second) {
                weights.push_back(weight);
            }
            std::discrete_distribution<> dist(weights.begin(), weights.end());
            return it->second[dist(gen)].first;
        }
        fetchFromApi(topic);
        return knowledgeBase[topic].empty() ? "" : knowledgeBase[topic][0].first;
    }

    void prioritizeKnowledge(const std::string& topic, const std::string& fact, double weight) {
        for (auto& [existingFact, existingWeight] : knowledgeBase[topic]) {
            if (existingFact == fact) {
                existingWeight = weight;
                break;
            }
        }
        normalizeWeights(topic);
    }

    void saveKnowledge(const std::string& filename) const {
        std::ofstream file(filename);
        if (file.is_open()) {
            for (const auto& [topic, facts] : knowledgeBase) {
                for (const auto& [fact, weight] : facts) {
                    file << topic << ":" << fact << ":" << weight << "\n";
                }
            }
            file.close();
        }
    }

    void loadKnowledge(const std::string& filename) {
        std::ifstream file(filename);
        std::string line;
        while (std::getline(file, line)) {
            size_t pos1 = line.find(':');
            size_t pos2 = line.rfind(':');
            if (pos1 != std::string::npos && pos2 != std::string::npos && pos1 != pos2) {
                std::string topic = line.substr(0, pos1);
                std::string fact = line.substr(pos1 + 1, pos2 - pos1 - 1);
                double weight = std::stod(line.substr(pos2 + 1));
                knowledgeBase[topic].push_back({fact, weight});
            }
        }
        file.close();
        for (auto& [topic, facts] : knowledgeBase) {
            normalizeWeights(topic);
        }
    }
};

class SubMenu {
public:
    std::string name;
    std::vector<std::unique_ptr<MenuItem>> items;
    std::vector<std::unique_ptr<SubMenu>> subMenus;

    SubMenu(const std::string& n) : name(n) {}

    void addItem(const std::string& name, std::function<void()> action, bool enabled = true) {
        items.push_back(std::make_unique<MenuItem>(name, action, enabled));
    }

    void addSubMenu(std::unique_ptr<SubMenu> subMenu) {
        subMenus.push_back(std::move(subMenu));
    }
};

class MenuSystem {
private:
    std::string title;
    bool isActive;
    sqlite3* db;
    std::string apiEndpoint;
    ResourceCache resourceCache;
    KnowledgePool knowledgePool;
    std::vector<std::unique_ptr<SubMenu>> mainMenu;
    bool developerMode;

    void initDatabase() {
        if (sqlite3_open("game_state.db", &db) != SQLITE_OK) {
            std::cerr << "Cannot open database: " << sqlite3_errmsg(db) << "\n";
            return;
        }
        const char* sql = "CREATE TABLE IF NOT EXISTS game_state (key TEXT PRIMARY KEY, value TEXT);";
        char* errMsg = nullptr;
        if (sqlite3_exec(db, sql, nullptr, nullptr, &errMsg) != SQLITE_OK) {
            std::cerr << "SQL error: " << errMsg << "\n";
            sqlite3_free(errMsg);
        }
    }

    void displayMenu(const SubMenu* menu, int depth = 0) const {
        std::cout << "\n=== " << std::string(depth * 2, ' ') << menu->name << " ===\n";
        for (size_t i = 0; i < menu->items.size(); ++i) {
            const auto& item = menu->items[i];
            std::cout << std::string(depth * 2, ' ') << i + 1 << ". " << item->name
                      << " (" << (item->enabled ? "Enabled" : "Disabled") << ")\n";
        }
        for (size_t i = 0; i < menu->subMenus.size(); ++i) {
            std::cout << std::string(depth * 2, ' ') << menu->items.size() + i + 1 << ". "
                      << menu->subMenus[i]->name << " (Sub-Menu)\n";
        }
        std::cout << std::string(depth * 2, ' ') << "0. " << (depth == 0 ? "Exit" : "Back") << "\n";
    }

    void logAction(const std::string& action) const {
        std::ofstream log("menu_log.txt", std::ios::app);
        if (log.is_open()) {
            time_t now = time(nullptr);
            log << ctime(&now) << ": " << action << "\n";
            log.close();
        }
    }

    void saveState(const std::string& key, const std::string& value) {
        std::string sql = "INSERT OR REPLACE INTO game_state (key, value) VALUES (?, ?);";
        sqlite3_stmt* stmt;
        if (sqlite3_prepare_v2(db, sql.c_str(), -1, &stmt, nullptr) == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, key.c_str(), -1, SQLITE_STATIC);
            sqlite3_bind_text(stmt, 2, value.c_str(), -1, SQLITE_STATIC);
            if (sqlite3_step(stmt) != SQLITE_DONE) {
                std::cerr << "SQL error: " << sqlite3_errmsg(db) << "\n";
            }
            sqlite3_finalize(stmt);
        } else {
            std::cerr << "SQL error: " << sqlite3_errmsg(db) << "\n";
        }
    }

    std::string loadState(const std::string& key, const std::string& defaultValue) const {
        std::string sql = "SELECT value FROM game_state WHERE key = ?;";
        sqlite3_stmt* stmt;
        std::string result = defaultValue;
        if (sqlite3_prepare_v2(db, sql.c_str(), -1, &stmt, nullptr) == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, key.c_str(), -1, SQLITE_STATIC);
            if (sqlite3_step(stmt) == SQLITE_ROW) {
                result = reinterpret_cast<const char* >(sqlite3_column_text(stmt, 0));
            }
            sqlite3_finalize(stmt);
        }
        return result;
    }

    void clearState() {
        const char* sql = "DELETE FROM game_state;";
        char* errMsg = nullptr;
        if (sqlite3_exec(db, sql, nullptr, nullptr, &errMsg) != SQLITE_OK) {
            std::cerr << "SQL error: " << errMsg << "\n";
            sqlite3_free(errMsg);
        }
    }

    std::string adjustDifficulty(int playerSkill) const {
        if (playerSkill < 3) return "Easy";
        if (playerSkill < 6) return "Medium";
        if (playerSkill < 9) return "Hard";
        return "Expert";
    }

    void applyKnowledgeToAction(const std::string& actionId) {
        std::string knowledge = knowledgePool.getKnowledge(actionId);
        if (!knowledge.empty()) {
            std::cout << "Applying knowledge: " << knowledge << "\n";
            saveState("action_knowledge_" + actionId, knowledge);
        }
    }

    void navigateMenu(SubMenu* currentMenu, int depth = 0) {
        while (isActive) {
            displayMenu(currentMenu, depth);
            int choice = getInput<int>("Enter choice: ");
            applyKnowledgeToAction(currentMenu->name + "_choice_" + std::to_string(choice));
            if (choice == 0) {
                if (depth == 0) {
                    isActive = false;
                }
                return;
            }
            if (choice > 0 && choice <= static_cast<int>(currentMenu->items.size())) {
                auto& item = currentMenu->items[choice - 1];
                if (item->enabled) {
                    item->action();
                    logAction(item->name);
                } else {
                    std::cout << "Item disabled.\n";
                }
            } else if (choice > static_cast<int>(currentMenu->items.size()) &&
                       choice <= static_cast<int>(currentMenu->items.size() + currentMenu->subMenus.size())) {
                navigateMenu(currentMenu->subMenus[choice - currentMenu->items.size() - 1].get(), depth + 1);
            } else {
                std::cout << "Invalid choice.\n";
            }
        }
    }

    void addMainBranch(const std::string& name) {
        mainMenu.push_back(std::make_unique<SubMenu>(name));
    }

public:
    MenuSystem(const std::string& title = "Apocalittz Main Menu",
               const std::string& apiKey = std::getenv("XAI_API_KEY") ? std::getenv("XAI_API_KEY") : "")
        : title(title), isActive(true), apiEndpoint("https://api.x.ai/v1/chat/completions"),
          knowledgePool(apiEndpoint, apiKey), developerMode(false) {
        initDatabase();
        initializeMenuStructure();
    }

    ~MenuSystem() {
        sqlite3_close(db);
    }

    void initializeMenuStructure() {
        mainMenu.clear();
        addMainBranch("Main Menu");

        auto gameplay = std::make_unique<SubMenu>("Gameplay");
        gameplay->addItem("Start New Mission", [this]() {
            fetchDynamicContent("mission");
        });
        gameplay->addItem("Load Mission", [this]() {
            std::cout << "Loading mission: " << loadState("last_mission", "None") << "\n";
        }, false);
        gameplay->addItem("Difficulty Settings", [this]() {
            int skill = std::stoi(loadState("player_skill", "1"));
            std::string newDifficulty = adjustDifficulty(skill + 1);
            saveState("difficulty", newDifficulty);
            std::cout << "Difficulty set to: " << newDifficulty << "\n";
        });
        gameplay->addItem("Mission History", [this]() {
            std::cout << "Mission history: " << loadState("last_mission", "None") << "\n";
        });
        gameplay->addItem("Faction Alliances", [this]() {
            std::cout << "Current faction standing: " << loadState("npc_relationship", "neutral") << "\n";
        });
        mainMenu.back()->addSubMenu(std::move(gameplay));

        auto charCustom = std::make_unique<SubMenu>("Character Customization");
        auto appearance = std::make_unique<SubMenu>("Appearance");
        appearance->addItem("Change Outfit", []() { std::cout << "Outfit changed.\n"; });
        appearance->addItem("Weapon Skins", []() { std::cout << "Weapon skin updated.\n"; });
        appearance->addItem("Character Model", []() { std::cout << "Character model updated.\n"; });
        charCustom->addSubMenu(std::move(appearance));
        auto skills = std::make_unique<SubMenu>("Skill Tree");
        skills->addItem("Combat Skill", [this]() {
            saveState("combat_skill", std::to_string(std::stoi(loadState("combat_skill", "0")) + 1));
            std::cout << "Combat skill upgraded to: " << loadState("combat_skill", "0") << "\n";
        });
        skills->addItem("Stealth Skill", [this]() {
            saveState("stealth_skill", std::to_string(std::stoi(loadState("stealth_skill", "0")) + 1));
            std::cout << "Stealth skill upgraded to: " << loadState("stealth_skill", "0") << "\n";
        });
        skills->addItem("Tech Skill", [this]() {
            saveState("tech_skill", std::to_string(std::stoi(loadState("tech_skill", "0")) + 1));
            std::cout << "Tech skill upgraded to: " << loadState("tech_skill", "0") << "\n";
        });
        skills->addItem("Survival Skill", [this]() {
            saveState("survival_skill", std::to_string(std::stoi(loadState("survival_skill", "0")) + 1));
            std::cout << "Survival skill upgraded to: " << loadState("survival_skill", "0") << "\n";
        });
        charCustom->addSubMenu(std::move(skills));
        charCustom->addItem("Perks", [this]() {
            saveState("perks", loadState("perks", "") + ",NewPerk");
            std::cout << "Perk unlocked: " << loadState("perks", "None") << "\n";
        });
        charCustom->addItem("Loadouts", [this]() {
            saveState("loadout", "Primary:AssaultRifle,Secondary:Pistol");
            std::cout << "Loadout configured: " << loadState("loadout", "None") << "\n";
        });
        mainMenu.back()->addSubMenu(std::move(charCustom));

        auto inventory = std::make_unique<SubMenu>("Inventory");
        inventory->addItem("View Inventory", [this]() {
            std::cout << "Inventory: " << loadState("inventory", "Empty") << "\n";
        });
        inventory->addItem("Add Item", [this]() {
            saveState("inventory", loadState("inventory", "") + ",NewItem");
            std::cout << "Item added: " << loadState("inventory", "Empty") << "\n";
        });
        inventory->addItem("Remove Item", [this]() {
            saveState("inventory", "Empty");
            std::cout << "Inventory cleared.\n";
        });
        auto crafting = std::make_unique<SubMenu>("Crafting");
        crafting->addItem("Craft Weapon", [this]() {
            saveState("inventory", loadState("inventory", "") + ",CraftedWeapon");
            std::cout << "Weapon crafted.\n";
        });
        crafting->addItem("Craft Armor", [this]() {
            saveState("inventory", loadState("inventory", "") + ",CraftedArmor");
            std::cout << "Armor crafted.\n";
        });
        inventory->addSubMenu(std::move(crafting));
        inventory->addItem("Trade", [this]() {
            std::string rel = loadState("npc_relationship", "neutral");
            std::cout << "Trading with NPC, relationship: " << rel << "\n";
            if (rel == "friendly") {
                saveState("inventory", loadState("inventory", "") + ",TradeBonus");
                std::cout << "Trade bonus received.\n";
            }
        });
        mainMenu.back()->addSubMenu(std::move(inventory));

        auto social = std::make_unique<SubMenu>("Social");
        social->addItem("NPC Relationships", [this]() {
            std::cout << "NPC standings: " << loadState("npc_relationship", "neutral") << "\n";
        });
        social->addItem("Send Message", [this]() {
            std::string message = getInput<std::string>("Enter message: ");
            saveState("last_message", message);
            std::cout << "Message sent to NPC: " << message << "\n";
        });
        social->addItem("Form Alliance", [this]() {
            saveState("npc_relationship", "friendly");
            std::cout << "Alliance formed with faction.\n";
        });
        social->addItem("Recruit Companion", [this]() {
            saveState("companions", loadState("companions", "") + ",NewCompanion");
            std::cout << "Companion recruited: " << loadState("companions", "None") << "\n";
        });
        social->addItem("Prioritize Knowledge", [this]() {
            std::string topic = getInput<std::string>("Enter topic: ");
            std::string fact = getInput<std::string>("Enter fact to prioritize: ");
            double weight = getInput<double>("Enter weight (0.0-1.0): ");
            knowledgePool.prioritizeKnowledge(topic, fact, weight);
            std::cout << "Knowledge prioritized for " << topic << ".\n";
        });
        mainMenu.back()->addSubMenu(std::move(social));

        auto settings = std::make_unique<SubMenu>("Settings");
        auto audio = std::make_unique<SubMenu>("Audio");
        audio->addItem("Music Volume", [this]() {
            saveState("music_volume", getInput<std::string>("Enter volume (0-100): "));
            std::cout << "Music volume set to: " << loadState("music_volume", "50") << "\n";
        });
        audio->addItem("SFX Volume", [this]() {
            saveState("sfx_volume", getInput<std::string>("Enter volume (0-100): "));
            std::cout << "SFX volume set to: " << loadState("sfx_volume", "50") << "\n";
        });
        settings->addSubMenu(std::move(audio));
        settings->addItem("Graphics", [this]() {
            saveState("graphics_quality", getInput<std::string>("Enter quality (Low/Medium/High): "));
            std::cout << "Graphics quality set to: " << loadState("graphics_quality", "Medium") << "\n";
        });
        settings->addItem("Controls", [this]() {
            saveState("controls", "Custom");
            std::cout << "Controls customized.\n";
        });
        settings->addItem("Accessibility", [this]() {
            saveState("accessibility", "Enabled");
            std::cout << "Accessibility settings updated.\n";
        });
        mainMenu.back()->addSubMenu(std::move(settings));

        auto devOptions = std::make_unique<SubMenu>("Developer Options");
        auto aiTuning = std::make_unique<SubMenu>("AI Tuning");
        aiTuning->addItem("Aggressiveness", [this]() {
            saveState("npc_aggressiveness", getInput<std::string>("Enter aggression (1-10): "));
            std::cout << "NPC aggressiveness set to: " << loadState("npc_aggressiveness", "5") << "\n";
        });
        aiTuning->addItem("Learning Rate", [this]() {
            saveState("ai_learning_rate", getInput<std::string>("Enter learning rate (0.0-1.0): "));
            std::cout << "AI learning rate set to: " << loadState("ai_learning_rate", "0.1") << "\n";
        });
        aiTuning->addItem("Behavior Profiles", [this]() {
            saveState("npc_profile", getInput<std::string>("Enter profile (Aggressive/Defensive/Neutral): "));
            std::cout << "NPC archetype set to: " << loadState("npc_profile", "Neutral") << "\n";
        });
        aiTuning->addItem("DeepSearch Mode", [this]() {
            saveState("deep_search", "true");
            knowledgePool.getKnowledge("deep_search");
            std::cout << "DeepSearch enabled.\n";
        });
        aiTuning->addItem("Think Mode", [this]() {
            saveState("think_mode", "true");
            knowledgePool.getKnowledge("think_mode");
            std::cout << "Think Mode enabled.\n";
        });
        aiTuning->addItem("Reset AI Models", [this]() {
            saveState("npc_aggressiveness", "5");
            saveState("ai_learning_rate", "0.1");
            saveState("npc_profile", "Neutral");
            std::cout << "AI models reset.\n";
        });
        aiTuning->addItem("NPC Decision Weights", [this]() {
            saveState("decision_weights", getInput<std::string>("Enter weights (JSON format): "));
            std::cout << "Decision weights updated.\n";
        });
        devOptions->addSubMenu(std::move(aiTuning));
        auto debugTools = std::make_unique<SubMenu>("Debug Tools");
        debugTools->addItem("Log Game State", [this]() {
            sqlite3_stmt* stmt;
            std::string sql = "SELECT key, value FROM game_state;";
            if (sqlite3_prepare_v2(db, sql.c_str(), -1, &stmt, nullptr) == SQLITE_OK) {
                while (sqlite3_step(stmt) == SQLITE_ROW) {
                    std::cout << sqlite3_column_text(stmt, 0) << ": " << sqlite3_column_text(stmt, 1) << "\n";
                }
                sqlite3_finalize(stmt);
            } else {
                std::cerr << "SQL error: " << sqlite3_errmsg(db) << "\n";
            }
        });
        debugTools->addItem("Clear Cache", [this]() {
            resourceCache.clearAll();
            std::cout << "Cache cleared.\n";
        });
        debugTools->addItem("Simulate Crash", []() { throw std::runtime_error("Simulated crash"); });
        debugTools->addItem("Toggle Debug Logs", [this]() {
            saveState("debug_logs", loadState("debug_logs", "false") == "true" ? "false" : "true");
            std::cout << "Debug logs " << (loadState("debug_logs", "false") == "true" ? "enabled" : "disabled") << ".\n";
        });
        debugTools->addItem("Memory Usage", []() { std::cout << "Memory usage: 128MB (placeholder).\n"; });
        debugTools->addItem("Performance Profiler", []() { std::cout << "Profiler started.\n"; });
        devOptions->addSubMenu(std::move(debugTools));
        auto contentEditor = std::make_unique<SubMenu>("Content Editor");
        contentEditor->addItem("Add Custom Mission", [this]() {
            fetchDynamicContent("custom_mission");
            std::cout << "Custom mission added.\n";
        });
        contentEditor->addItem("Edit NPC Dialogue", [this]() {
            saveState("npc_dialogue", getInput<std::string>("Enter new dialogue: "));
            std::cout << "NPC dialogue updated: " << loadState("npc_dialogue", "Default") << "\n";
        });
        contentEditor->addItem("Generate Map", [this]() {
            saveState("map", "NewMap_" + std::to_string(time(nullptr)));
            std::cout << "Map generated: " << loadState("map", "None") << "\n";
        });
        contentEditor->addItem("Add Faction", [this]() {
            saveState("factions", loadState("factions", "") + ",NewFaction");
            std::cout << "Faction added: " << loadState("factions", "None") << "\n";
        });
        contentEditor->addItem("Create Item", [this]() {
            saveState("inventory", loadState("inventory", "") + ",CustomItem");
            std::cout << "Item created: " << loadState("inventory", "Empty") << "\n";
        });
        contentEditor->addItem("Edit Environment", [this]() {
            saveState("environment", getInput<std::string>("Enter environment (e.g., Wasteland): "));
            std::cout << "Environment updated: " << loadState("environment", "Default") << "\n";
        });
        devOptions->addSubMenu(std::move(contentEditor));
        auto systemOverrides = std::make_unique<SubMenu>("System Overrides");
        systemOverrides->addItem("Unlock All Missions", [this]() {
            saveState("missions_unlocked", "true");
            std::cout << "All missions unlocked.\n";
        });
        systemOverrides->addItem("Reset Game State", [this]() {
            clearState();
            std::cout << "Game state reset.\n";
        });
        systemOverrides->addItem("Max Resources", [this]() {
            saveState("resources", "9999");
            std::cout << "Resources set to 9999.\n";
        });
        systemOverrides->addItem("God Mode", [this]() {
            saveState("god_mode", "true");
            std::cout << "God mode enabled.\n";
        });
        systemOverrides->addItem("Fast Travel", [this]() {
            saveState("fast_travel", "true");
            std::cout << "Fast travel unlocked.\n";
        });
        systemOverrides->addItem("Time Manipulation", [this]() {
            saveState("game_time", getInput<std::string>("Enter game time (e.g., 12:00): "));
            std::cout << "Game time set to: " << loadState("game_time", "00:00") << "\n";
        });
        devOptions->addSubMenu(std::move(systemOverrides));
        auto analytics = std::make_unique<SubMenu>("Analytics Dashboard");
        analytics->addItem("Player Stats", [this]() {
            std::cout << "Player skill: " << loadState("player_skill", "1") << "\n";
            std::cout << "Combat skill: " << loadState("combat_skill", "0") << "\n";
            std::cout << "Stealth skill: " << loadState("stealth_skill", "0") << "\n";
            std::cout << "Tech skill: " << loadState("tech_skill", "0") << "\n";
            std::cout << "Survival skill: " << loadState("survival_skill", "0") << "\n";
        });
        analytics->addItem("NPC Interactions", [this]() {
            std::cout << "NPC interactions: " << loadState("npc_relationship", "neutral") << "\n";
            std::cout << "Last message: " << loadState("last_message", "None") << "\n";
        });
        analytics->addItem("Mission Analytics", [this]() {
            int successCount = std::stoi(loadState("mission_success_count", "0"));
            int totalMissions = std::stoi(loadState("mission_total_count", "0"));
            double rate = totalMissions > 0 ? (successCount * 100.0 / totalMissions) : 0.0;
            std::cout << "Mission success rate: " << rate << "%\n";
        });
        analytics->addItem("Resource Usage", [this]() {
            std::cout << "Resources: " << loadState("resources", "0") << "\n";
        });
        analytics->addItem("Player Behavior", [this]() {
            std::string profile = std::stoi(loadState("npc_aggressiveness", "5")) > 7 ? "Aggressive" :
                                 std::stoi(loadState("npc_aggressiveness", "5")) < 3 ? "Passive" : "Balanced";
            std::cout << "Player behavior: " << profile << "\n";
        });
        devOptions->addSubMenu(std::move(analytics));
        auto networkTools = std::make_unique<SubMenu>("Network Tools");
        networkTools->addItem("API Test", [this]() {
            knowledgePool.getKnowledge("api_test");
            std::cout << "API test completed.\n";
        });
        networkTools->addItem("Cache Sync", [this]() {
            resourceCache.clearExpired();
            knowledgePool.getKnowledge("cache_sync");
            std::cout << "Cache synced.\n";
        });
        networkTools->addItem("Data Mining", [this]() {
            knowledgePool.getKnowledge("data_mining");
            std::cout << "Data mining initiated.\n";
        });
        networkTools->addItem("Knowledge Import", [this]() {
            std::string fact = getInput<std::string>("Enter knowledge fact: ");
            knowledgePool.addKnowledge("imported", fact, 1.0);
            std::cout << "Knowledge imported: " << fact << "\n";
        });
        devOptions->addSubMenu(std::move(networkTools));
        auto moddingTools = std::make_unique<SubMenu>("Modding Tools");
        moddingTools->addItem("Script Editor", [this]() {
            saveState("custom_script", getInput<std::string>("Enter script content: "));
            std::cout << "Script editor saved: " << loadState("custom_script", "None") << "\n";
        });
        moddingTools->addItem("Asset Importer", [this]() {
            saveState("assets", loadState("assets", "") + ",NewAsset");
            std::cout << "Asset imported: " << loadState("assets", "None") << "\n";
        });
        moddingTools->addItem("Mod Installer", [this]() {
            saveState("mods", loadState("mods", "") + ",NewMod");
            std::cout << "Mod installed: " << loadState("mods", "None") << "\n";
        });
        devOptions->addSubMenu(std::move(moddingTools));
        if (developerMode) {
            mainMenu.back()->addSubMenu(std::move(devOptions));
        }

        mainMenu.back()->addItem("Toggle Developer Mode", [this]() {
            toggleDeveloperMode();
        });
        mainMenu.back()->addItem("Save Game", [this]() {
            saveGameState("game_state.txt");
            std::cout << "Game saved.\n";
        });
        mainMenu.back()->addItem("Load Game", [this]() {
            loadGameState("game_state.txt");
            std::cout << "Game loaded.\n";
        });
    }

    void fetchDynamicContent(const std::string& contentType) {
        std::string cacheKey = "dynamic_" + contentType;
        std::string cachedContent = resourceCache.getResource(cacheKey);
        if (!cachedContent.empty()) {
            std::cout << "Using cached content for " << contentType << ": " << cachedContent << "\n";
            mainMenu[0]->items.push_back(std::make_unique<MenuItem>(
                cachedContent, [this, cachedContent]() {
                    startFreelancingMission(cachedContent);
                }));
            return;
        }

        std::vector<std::string> missionTemplates = {
            "Survive zombie horde in ${location}",
            "Infiltrate ${faction} base",
            "Rescue survivor from ${environment}"
        };
        std::vector<std::string> locations = {"abandoned city", "desert outpost", "toxic wasteland"};
        std::vector<std::string> factions = {"rogue militia", "cyber cult", "mutant clan"};
        std::vector<std::string> environments = {"ruined skyscraper", "underground bunker", "irradiated forest"};

        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_int_distribution<> dist(0, 2);

        std::string mission = missionTemplates[dist(gen)];
        if (mission.find("${location}") != std::string::npos) {
            mission.replace(mission.find("${location}"), 11, locations[dist(gen)]);
        }
        if (mission.find("${faction}") != std::string::npos) {
            mission.replace(mission.find("${faction}"), 10, factions[dist(gen)]);
        }
        if (mission.find("${environment}") != std::string::npos) {
            mission.replace(mission.find("${environment}"), 13, environments[dist(gen)]);
        }

        resourceCache.cacheResource(cacheKey, mission);
        mainMenu[0]->items.push_back(std::make_unique<MenuItem>(
            mission, [this, mission]() {
                startFreelancingMission(mission);
            }));
        std::cout << "Generated and cached mission: " << mission << "\n";
        applyKnowledgeToAction(mission);
    }

    void startFreelancingMission(const std::string& missionId) {
        applyKnowledgeToAction(missionId);
        std::string cacheKey = "mission_" + missionId;
        std::string cachedData = resourceCache.getResource(cacheKey);
        if (!cachedData.empty()) {
            std::cout << "Using cached mission data: " << cachedData << "\n";
        } else {
            cachedData = "Mission data for " + missionId;
            resourceCache.cacheResource(cacheKey, cachedData);
        }

        saveState("last_mission", missionId);
        int playerSkill = std::stoi(loadState("player_skill", "1"));
        std::string difficulty = adjustDifficulty(playerSkill);
        std::cout << "Starting mission: " << missionId << " (Difficulty: " << difficulty << ")\n";

        std::string npcResponse = loadState("npc_relationship", "neutral");
        if (npcResponse == "hostile") {
            std::cout << "NPCs are aggressive due to past encounters!\n";
        } else if (npcResponse == "friendly") {
            std::cout << "NPCs offer assistance based on prior trust.\n";
        }

        saveState("mission_total_count", std::to_string(std::stoi(loadState("mission_total_count", "0")) + 1));
        logAction("Started mission: " + missionId);
    }

    void completeFreelancingMission(const std::string& missionId) {
        applyKnowledgeToAction(missionId);
        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_int_distribution<> dist(1, 100);
        int successChance = std::stoi(loadState("player_skill", "1")) * 10;
        bool success = dist(gen) <= successChance;

        if (success) {
            saveState("npc_relationship", "friendly");
            saveState("player_skill", std::to_string(std::stoi(loadState("player_skill", "1")) + 1));
            saveState("mission_success_count", std::to_string(std::stoi(loadState("mission_success_count", "0")) + 1));
            std::cout << "Mission " << missionId << " completed successfully!\n";
        } else {
            saveState("npc_relationship", "hostile");
            std::cout << "Mission " << missionId << " failed. NPCs are now hostile.\n";
        }
        logAction("Completed mission: " + missionId + (success ? " (Success)" : " (Failed)"));
    }

    void initializeAI(const std::string& apiKey, const std::string& model = "grok-beta") {
        std::cout << "Initializing AI with model: " << model << " and API key: " << apiKey << "\n";
    }

    void toggleDeveloperMode() {
        developerMode = !developerMode;
        initializeMenuStructure();
        std::cout << "Developer mode " << (developerMode ? "enabled" : "disabled") << ".\n";
    }

    void run() {
        while (isActive) {
            displayMenu(mainMenu[0].get());
            int choice = getInput<int>("Enter choice: ");
            applyKnowledgeToAction("main_menu_choice_" + std::to_string(choice));
            if (choice == 0) {
                isActive = false;
                continue;
            }
            if (choice > 0 && choice <= static_cast<int>(mainMenu[0]->items.size())) {
                auto& item = mainMenu[0]->items[choice - 1];
                if (item->enabled) {
                    item->action();
                    logAction(item->name);
                } else {
                    std::cout << "Item disabled.\n";
                }
            } else if (choice > static_cast<int>(mainMenu[0]->items.size()) &&
                       choice <= static_cast<int>(mainMenu[0]->items.size() + mainMenu.size() - 1)) {
                navigateMenu(mainMenu[choice - mainMenu[0]->items.size()].get(), 1);
            } else {
                std::cout << "Invalid choice.\n";
            }
        }
    }

    void addItem(const std::string& name, std::function<void()> action, bool enabled = true) {
        mainMenu[0]->items.push_back(std::make_unique<MenuItem>(name, action, enabled));
    }

    void removeItem(const std::string& name) {
        for (auto& menu : mainMenu) {
            menu->items.erase(std::remove_if(menu->items.begin(), menu->items.end(),
                [&name](const auto& item) { return item->name == name; }), menu->items.end());
        }
    }

    void setItemEnabled(const std::string& name, bool enabled) {
        for (auto& menu : mainMenu) {
            for (auto& item : menu->items) {
                if (item->name == name) {
                    item->enabled = enabled;
                    return;
                }
            }
        }
    }

    void saveGameState(const std::string& filename) const {
        std::ofstream file(filename);
        if (file.is_open()) {
            sqlite3_stmt* stmt;
            std::string sql = "SELECT key, value FROM game_state;";
            if (sqlite3_prepare_v2(db, sql.c_str(), -1, &stmt, nullptr) == SQLITE_OK) {
                while (sqlite3_step(stmt) == SQLITE_ROW) {
                    file << sqlite3_column_text(stmt, 0) << "=" << sqlite3_column_text(stmt, 1) << "\n";
                }
                sqlite3_finalize(stmt);
            }
            file.close();
        }
    }

    void loadGameState(const std::string& filename) {
        clearState();
        std::ifstream file(filename);
        std::string line;
        while (std::getline(file, line)) {
            size_t pos = line.find('=');
            if (pos != std::string::npos) {
                saveState(line.substr(0, pos), line.substr(pos + 1));
            }
        }
        file.close();
    }

private:
    template<typename T>
    T getInput(const std::string& prompt) const {
        std::cout << prompt;
        T value;
        if constexpr (std::is_same_v<T, std::string>) {
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
            std::getline(std::cin, value);
        } else {
            std::cin >> value;
            std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        }
        return value;
    }
};

} // namespace Apocalittz

#endif // APOCALITTZ_RESOURCE_KNOWLEDGE_H
