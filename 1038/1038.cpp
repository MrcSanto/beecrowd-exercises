#include <iostream>
#include <iomanip>
#include <map>

int main(){
    std::map<int, float> t = {
        {1, 4.00}, 
        {2, 4.50},
        {3, 5.00},
        {4, 2.00},
        {5, 1.50}
    }; 

    int x, y;
    std::cin >> x >> y;

    float result = t[x] * y;

    std::cout << "Total: R$ " << std::fixed << std::setprecision(2) << result << std::endl;

    return 0;
}