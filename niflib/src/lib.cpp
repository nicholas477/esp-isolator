#include "lib.h"
#include "niflib.h"
#include "obj/NiObject.h"
#include "MatTexCollection.h"
#include "obj/NiAVObject.h"
#include "obj/NiSourceTexture.h"
#include "obj/NiImage.h"
#include <filesystem>

using namespace Niflib;

void rust_test()
{
    try
    {
        const std::string nifFilePath = "I:\\SteamLibrary\\steamapps\\common\\Morrowind\\Data Files\\meshes\\tr\\f\\tr_f_fresco_flower_01.nif"; // Replace with your NIF file path

        std::cout << "Reading NIF file: " << nifFilePath << std::endl;

        std::vector<std::string> textureFilepaths;

        NifInfo info;

        unsigned ver = GetNifVersion(nifFilePath);
        if (IsSupportedVersion(ver) == false)
        {
            cout << "Unsupported niflib version: " << hex << ver
                 << endl;
            return;
        }

        // Ref<NiObject> root = ReadNifTree(nifFilePath, &info);

        // // 2. Cast to NiAVObject (the scene root)
        // NiAVObject *sceneRoot = dynamic_cast<NiAVObject *>(&(*root));

        // if (!sceneRoot)
        // {
        //     return; // Empty if not a valid scene
        // }

        // // 3. Create a material/texture collection from the scene
        // MatTexCollection materials(sceneRoot);

        // // 4. Iterate through all textures and collect filepaths
        // unsigned int numTextures = materials.GetNumTextures();
        // for (unsigned int i = 0; i < numTextures; ++i)
        // {
        //     TextureWrapper texture = materials.GetTexture(i);

        //     // Get the texture filename
        //     std::string filepath = texture.GetTextureFileName();

        //     // Only add if it's an external texture and filepath is not empty
        //     if (texture.IsTextureExternal() && !filepath.empty())
        //     {
        //         textureFilepaths.push_back(filepath);
        //     }
        // }
    }
    catch (const std::exception &e)
    {
        std::cerr << "Error reading NIF file: " << e.what() << std::endl;
    }
}

std::unique_ptr<std::vector<std::string>> rust_GetNifTextureFilepaths(const std::string &nifFilePath)
{
    try
    {
        std::cout << "Reading NIF file: " << nifFilePath << std::endl;

        std::vector<std::string> textureFilepaths;

        NifInfo info;

        // std::ifstream file(nifFilePath);
        // if (file.good())
        // {
        //     std::cout << "The file exists!\n";
        // }

        unsigned ver = GetNifVersion(nifFilePath);
        if (IsSupportedVersion(ver) == false)
        {
            cout << "Unsupported niflib version: " << hex << ver
                 << endl;
            return nullptr;
        }

        Ref<NiObject> root = ReadNifTree(nifFilePath, &info);

        // 2. Cast to NiAVObject (the scene root)
        NiAVObject *sceneRoot = dynamic_cast<NiAVObject *>(&(*root));

        if (!sceneRoot)
        {
            return nullptr; // Empty if not a valid scene
        }

        // 3. Create a material/texture collection from the scene
        MatTexCollection materials(sceneRoot);

        // 4. Iterate through all textures and collect filepaths
        unsigned int numTextures = materials.GetNumTextures();
        for (unsigned int i = 0; i < numTextures; ++i)
        {
            TextureWrapper texture = materials.GetTexture(i);

            // Get the texture filename
            std::string filepath = texture.GetTextureFileName();

            // Only add if it's an external texture and filepath is not empty
            if (texture.IsTextureExternal() && !filepath.empty())
            {
                textureFilepaths.push_back(filepath);
            }
        }

        return std::make_unique<std::vector<std::string>>(std::move(textureFilepaths));
    }
    catch (const std::exception &e)
    {
        std::cerr << "Error reading NIF file: " << e.what() << std::endl;
        // Handle exceptions, possibly log the error
        return nullptr; // Return empty if an error occurs
    }
}