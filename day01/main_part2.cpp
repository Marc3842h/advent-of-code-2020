#include <iostream>
#include <vector>
#include <fstream>
#include <sstream>

int main() {
    std::vector<int32_t> lines;
    std::ifstream file("input.txt");

    std::string l;
    while (std::getline(file, l)) {
        lines.emplace_back(std::stoi(l));
    }

    for (int32_t line : lines) {
        for (int32_t line2 : lines) {
            for (int32_t line3 : lines) {
                if (line + line2 + line3 == 2020) {
                    std::cout << line << " * " << line2 << " * " << line3 << " = " << (line * line2 * line3) << std::endl;
                }
            }
        }
    }

    return 0;
}
