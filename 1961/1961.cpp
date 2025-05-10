#include <iostream>
#include <vector>

int meuAbs(int x) {
    return x < 0 ? -x : x;
}

int main() {
    int p, n, altura;
    std::cin >> p >> n;

    std::vector<int> alturas;
    alturas.reserve(n); // evita realocações desnecessárias

    for (int i = 0; i < n; i++) {
        std::cin >> altura;
        alturas.push_back(altura);
    }

    for (int i = 1; i < n; i++) {
        int diff = meuAbs(alturas[i] - alturas[i - 1]);
        if (diff > p) {
            std::cout << "GAME OVER" << std::endl;
            return 0;
        }
    }

    std::cout << "YOU WIN" << std::endl;
    return 0;
}
