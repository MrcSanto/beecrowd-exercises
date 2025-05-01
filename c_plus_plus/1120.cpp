#include <iostream>
#include <string>

std::string remove_char(std::string &str, char &d){
    std::string ret = "";
    for (char c: str){
        if (d != c) ret += c;
    }
    return ret;
}

int main(){
    char d;
    std::string n;
    while(std::cin >> d >> n){
        std::string str_res = remove_char(n, d);
        if (str_res.empty()) continue;

        std::cout << std::stoi(remove_char(n, d)) << std::endl;
    };

    return 0;
}