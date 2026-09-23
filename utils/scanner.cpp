#include <string>
#include <iostream>
#include <filesystem>

namespace fs  = std::filesystem;

void scan( const fs::path& dir ) {
    std::cout << "Initializing the scan..." << '\n';

    // Scan the directory recursively
    for ( const auto& entry : fs::recursive_directory_iterator(dir) ) {

        // Check if it's a file
        if (not entry.is_directory()) {

            std::error_code ec;
            auto size = entry.file_size(ec);
            
            // Return only if it successfully scanned it
            if (not ec) {
                std::string returnValue = entry.path().string() + " " + std::to_string(size) + "Bytes";
                std::cout << returnValue << '\n';
            }
        }
    }
}
