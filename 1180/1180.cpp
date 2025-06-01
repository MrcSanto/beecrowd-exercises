#include <iostream>
#include <vector>

int main(){
    int n, min, pos;
    std::cin >> n;

    int vet[n];

    for (int i=0;i<n;i++){
        std::cin >> vet[i];
    }

    min = vet[0];
    for (int i=0;i<n;i++){
        if (vet[i] < min) {
            min = vet[i];
            pos = i;
        }
    }

    std::cout << "Menor valor: " << min << "\n";
    std::cout << "Posicao: " << pos << "\n";    

    return 0;
}