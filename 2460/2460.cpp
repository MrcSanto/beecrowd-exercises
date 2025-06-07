#include <iostream>
#include <map>


int main(){
    int n, m;
    std::map<int, int> fila_ingressos;
    std::cin >> n;

    int v[n];

    for (int i=0;i<n;i++){
        std::cin >> v[i];
        fila_ingressos[v[i]] = 1;
    }

    std::cin >> m;
    for (int i=0;i<m;i++){
        int a;
        std::cin >> a;
        fila_ingressos[a] = 0;
    }

    bool first = true;
    for (int i=0;i<n;i++){
        if (fila_ingressos[v[i]] == 1){
            if (!first) {
                std::cout << " ";
            }
            std::cout << v[i];
            first = false;
        }
    }
    std::cout << std::endl;

    return 0;
}