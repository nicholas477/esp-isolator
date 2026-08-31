#pragma once

#include <memory>
#include <vector>
#include <string>

void rust_test();
std::unique_ptr<std::vector<std::string>> rust_GetNifTextureFilepaths(const std::string &nifFilePath);