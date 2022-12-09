#include <iostream>
#include <stdlib.h>
#include <fstream>
#include <string>

int main(int argc, char* argv[])
{
    if (!argc < 2)
    {
        std::cout << "Rain Lang - v0.1.0a" << std::endl;
        std::cout << "\n\nTo use the compiler, add a path to your .rain file" << std::endl;
    }
    else
    {
        std::string code;
        std::ifstream CodeFile;

        for (int i = 1; i < argc; i++)
        {
            
            if (!CodeFile)
                std::cout << "File " << argv[i] << " does not exist. Use a source file." << std::endl;
            else
                break;
        }

        while (getline(CodeFile, code)) {
        
            std::cout << code;
        }

        
        CodeFile.close();
   
    }

}

