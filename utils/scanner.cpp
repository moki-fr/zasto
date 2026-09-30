#include <cstdint>
#include <iostream>
#include <filesystem>
#include <ostream>
#include <vector>
#include <algorithm>
#include <string>

namespace fs  = std::filesystem; 

auto scan( const fs::path& dir, int numFile) {

    // variables
    int filesScanned = 0;
    int filesNotScanned = 0;
    std::vector<std::pair<fs::path, uintmax_t>> table;

    std::cout << "Initializing the scan..." << '\n';

    // Scan the directory recursively
    for ( const auto& entry : fs::recursive_directory_iterator(dir) ) {

        // Check if it's a file
        if (!entry.is_directory()) {
            std::error_code ec;
            auto size = entry.file_size(ec);

            // Check if it successfully scanned the file
            if (!ec) {
                // count the number of files scanned
                filesScanned++;

                // Store the files and his size
                table.push_back({entry.path().string(), size});
                std::cout << "\r" << "Sucesefully scanned: " << filesScanned << " files scanned!" << std::flush;
            
            // Count the number of files the scanner couldn't acess
            } else {
                filesNotScanned++;

            }

        }

    }

    std::cout << std::endl;

    std::cout << "Couldn't scanned " << filesNotScanned << " other files!" << '\n';

    // Sort from the biggest file to the smallest
    std::sort(table.begin(), table.end(),
    [](const auto& a, const auto& b) {
        return a.second > b.second;
    });

    // Create the final table the the function will return 
    std::vector<std::string> returnTable;

    // Translate the tables to one string and put it inside the returnTable
    for (size_t i = 0; i < std::min(table.size(), size_t(numFile)); i++) {
        std::string toString = table[i].first.string() + " " + std::to_string(table[i].second);
        std::cout << toString << '\n';
    }

    std::cout << '\n' << "The " << dir << " directory has been sucessefully scanned!" << '\n';

    return returnTable;
}

// debug stuff
int main() {
    fs::path path = R"(/home/mint/)";
    scan(path, 1000);
}
