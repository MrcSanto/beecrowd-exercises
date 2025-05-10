#include <iostream>
#include <numeric>

int main(){
    int n, numerador, denominador;
    std::cin >> n;
    for (int i=0;i<n;i++){
        char op;
        int n1, d1, n2, d2;
        scanf("%d / %d %c %d / %d", &n1, &d1, &op, &n2, &d2);

        if (op == '+'){
            numerador = n1*d2 + n2*d1;
            denominador = d1 * d2;
        } else if (op == '-'){
            numerador = n1*d2 - n2*d1;
            denominador = d1 * d2;
        } else if (op == '*'){
            numerador = n1 * n2;
            denominador = d1 * d2;
        } else {
            numerador = n1 * d2;
            denominador = n2 * d1;
        }
        int mdc = std::gcd(numerador, denominador);
        std::cout << numerador << "/" << denominador << " = " << numerador/mdc << "/" << denominador/mdc << std::endl;;
    }

    return 0;
}