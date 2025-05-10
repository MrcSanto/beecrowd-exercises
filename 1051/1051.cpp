#include <iostream>
#include <iomanip>

int main(){
    double salario, resto, taxa = 0;
    
    double salarios_corte[3] = {4500.00, 3000.00, 2000.00};
    double aliquotas[3] = {0.28, 0.18, 0.08};

    std::cin >> salario;

    for(int i=0;i<3;i++){
        resto = salario - salarios_corte[i];
        if (resto > 0.00){
            taxa += (resto * aliquotas[i]);
            salario -= resto;
        }
    }

    if (taxa == 0.00){
        std::cout << "Isento" << std::endl;
    } else {
        std::cout << std::fixed << std::setprecision(2) << "R$ " << taxa << std::endl;
    }

    return 0;
}