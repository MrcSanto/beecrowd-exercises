#include <iostream>
#include <stack>

// podia tb ter usado uma queue, nesse caso funcionaria igual tb

int main(){
    std::string entrada; //string de entrada dos diamantes e areia
    std::stack<char> pilha; //pilha para armazenar
    int n, size, diamantes; 

    std::cin >> n;
    for(int i=0;i<n;i++){
        while(!pilha.empty()){  //while para limpar a pilha a cada loop do for
            pilha.pop();
        }
        std::cin >> entrada;

        diamantes = 0;
        size = entrada.length(); // para obter o tamanho da string, é usado no loop
        for(int i=0;i<size;i++){
            if(entrada[i] == '<'){ // se entrada for igual a <(abrindo) da um push
                pilha.push('<');
            }
            else if(entrada[i] == '>' && !pilha.empty()){ //e se for igual a >(fechando)
                diamantes++;                            // e a pilha estiver com 1 elemento (o <)
                pilha.pop();                            //incrementa diamantes e da um pop para outro loop
            }
        }
        std::cout << diamantes << std::endl;
    }
    
    return 0;
}