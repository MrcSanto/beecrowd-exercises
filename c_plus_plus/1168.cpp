#include <iostream>
#include <string>
#include <map>

std::map<char, int> t = {
    {'0', 6}, {'1', 2}, {'2', 5},
    {'3', 5}, {'4', 4}, {'5', 5},
    {'6', 6}, {'7', 3}, {'8', 7}, {'9', 6}
};

int main(){
    int n, l;
    std::string s;

    std::cin >> n;
    for (int i=0;i<n;i++){
        std::cin >> s;
        l = 0;
        for (char c: s){
            l += t[c];
        }
        std::cout << l << " leds" << std::endl;
    }

    return 0;
}