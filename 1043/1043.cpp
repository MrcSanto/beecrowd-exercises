#include <iostream>
#include <iomanip>

int main(){
    float a, b, c;
    std::cin >> a >> b >> c;

    if (a < b + c && b < a + c && c < a + b)
        std::cout << std::fixed << std::setprecision(1) << "Perimetro = " << a + b + c << std::endl;
    else
        std::cout << std::fixed << std::setprecision(1) << "Area = " << ((a + b) * c) / 2 << std::endl;

    return 0;
}