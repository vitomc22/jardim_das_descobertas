// Port de art.js: os mesmos desenhos vetoriais, gerados como SVG (std::string).
#pragma once
#include <string>
#include <vector>

namespace jardim::art {
std::string icon(const std::string& kind, const std::string& color = "");
std::string tile(const std::string& kind);
std::string garden();
std::string sock(const std::string& color, const std::string& pattern);
std::string cup(double amount);
std::string container(const std::string& place);
std::string tower(const std::vector<std::string>& colors);
std::string position(const std::string& dir);
std::string clock(int hour);
// Interpreta o id usado pelo image provider ("icon/flower/eb6d58", "garden"…).
std::string fromSpec(const std::string& spec);
}  // namespace jardim::art
