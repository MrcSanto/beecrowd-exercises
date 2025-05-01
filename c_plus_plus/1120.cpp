#include <iostream>

int main(){
    std::size_t p = 0;
    char D;
    std::string N, R;

    while (std::cin >> D >> N){
        if (D == '0' && N == "0") break;

        R = "";
        for (std::size_t i=0;i<N.length();i++){
            if (N[i] != D) R += N[i];
        }

        p = 0;
        while(p < R.length()){
            if (R[p] != '0') break;
            ++p;
        }

        if (p == R.length()) std::cout << 0 << std::endl;
        else {
            R = R.substr(p);
            std::cout << R << std::endl;
        }
    }

    return 0;
}