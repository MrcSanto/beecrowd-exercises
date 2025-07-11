#include <iostream>

int main() {
    int N;
    std::cin >> N;

    for (int i=0;i<N;i++){
        int r1, r2;
        std::cin >> r1 >> r2;

        std::cout << r1 + r2 << std::endl;
    }

    return 0;
}