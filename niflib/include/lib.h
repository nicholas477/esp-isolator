#pragma once

#include <memory>
#include <string>
#include <vector>

void rust_test();
std::unique_ptr<std::vector<std::string>> rust_GetNifTextureFilepaths(const std::string& nifFilePath);