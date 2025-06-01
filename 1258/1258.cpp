#include <iostream>
#include <algorithm>
#include <vector>

struct Camiseta{
    std::string nome;
    std::string cor;
    char tamanho;
};

// compare function
int comp (Camiseta& a, Camiseta& b){
    if (a.cor == b.cor) {
        if (a.tamanho == b.tamanho){
            return a.nome < b.nome;
        }
        return a.tamanho > b.tamanho;
    }
    return a.cor < b.cor;
}

int main(){
    int n;
    bool first; // flag para printar a quebra de linha
    std::vector<Camiseta> camisetas;

    first = true;
    while (std::cin >> n){
        if (n == 0) return 0;

        camisetas.clear();

        if (first) first = false;
        else std::cout << std::endl;

        for (int i=0;i<n;i++){
            Camiseta c;

            std::cin.ignore();

            std::getline(std::cin, c.nome); // getline pois aqui é um nome
            std::cin >> c.cor >> c.tamanho; // cin aqui pois são valores separados

            camisetas.push_back(c);
        }

        std::sort(camisetas.begin(), camisetas.end(), comp);
        
        // printando valores
        for (const auto& c: camisetas){
            std::cout << c.cor << " " << c.tamanho << " " << c.nome << std::endl;
        }
    }    


    return 0;
}