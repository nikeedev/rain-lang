#include <iostream>
#include <stdlib.h>
#include <fstream>
#include <string>

int correct_file = 0;

int main(int argc, char* argv[])
{
    for (int i = 1; i < argc; i++)
        std::cout << i << " " << argv[i] << std::endl;

    if (argc > 1)
    {
        std::cout << "Rain Lang - v0.1.0a" << std::endl;
        std::cout << "\nTo use the compiler, add a path to your .rain file" << std::endl;
    }
    else
    {
        std::string code;
        
        
        for (int i = 1; i < argc; i++)
        {
            std::ifstream CodeFile(argv[i]);   
            if (!CodeFile)
                std::cout << "File " << argv[i] << " does not exist. Use a source file." << std::endl;
            else
                correct_file = i;
                break;
        }
        

        std::ifstream CodeFile(argv[1]);

        if (!CodeFile)
            std::cout << "File " << argv[1] << " does not exist. Use a source file." << std::endl;

        while (getline(CodeFile, code))
        {
            std::cout << code;
        }

        
        CodeFile.close();
   
    }

}

