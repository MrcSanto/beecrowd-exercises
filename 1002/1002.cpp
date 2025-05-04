#include <iostream>
#include <iomanip>

int main(){
    double raio, pi = 3.14159;
    std::cin >> raio;

    std::cout << std::fixed << std::setprecision(4) << "A=" << raio * raio * pi << std::endl;

    return 0;
}