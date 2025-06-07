#include <iostream>
#include <unordered_set>
#include <map>
#include <vector>
#include <algorithm>


int main() {
    int p, a, r;
    
    while (std::cin >> p >> a >> r){
        if (p == 0 && a == 0 && r == 0) return 0;
        
        std::cin.ignore();

        std::unordered_set<std::string> perolas;

        std::string perola;
        for (int i=0;i<p;i++){
            std::getline(std::cin, perola);
            perolas.insert(perola);
        }

        std::map<std::string, int> contagem_erros;
        

        for (int i=0;i<a;i++){
            std::string nome_aluno;
            std::getline(std::cin, nome_aluno);
            int qtd = 0; // quantidade de erros

            for (int j=0;j<r;j++){
                std::string resposta;
                std::getline(std::cin, resposta);
                if (perolas.find(resposta) != perolas.end()){
                    ++qtd;
                }
            }
            // adicionando a quantidade de erros ao map criado anteriormente
            contagem_erros[nome_aluno] = qtd;
        }

        // descobre a quantidade máxima de erros ocorridos
        int max = 0;
        for (const auto& [nome, qtd] : contagem_erros){
            if (qtd > max) max = qtd;
        }

        // Junta os nomes
        std::vector<std::string> perdedores;
        for (const auto& [nome, qtd_erros] : contagem_erros){
            if (qtd_erros == max) perdedores.push_back(nome);
        }

        // ordenando os nomes para printar
        std::sort(perdedores.begin(), perdedores.end());

        // printando os perdedores no terminal
        for (size_t i=0;i<perdedores.size();i++){
            if (i > 0) std::cout << ", ";
            std::cout << perdedores[i];
        }
        std::cout << std::endl;

    }


    return 0;
}

