#include <iostream>

int NUM_CALLS;

int fibo(int n) {
    NUM_CALLS++;
    if (n == 0) return 0;
    if (n == 1) return 1;
    else return fibo(n-1) + fibo(n-2);
}

int main() {
    int n, x;
    std::cin >> n;
    for (int i = 0; i < n; i++) {
        std::cin >> x;
        NUM_CALLS = 0;
        int resultado = fibo(x);
        std::cout << "fib(" << x << ") = " << NUM_CALLS - 1 << " calls = " << resultado << std::endl;
    }
    return 0;
}
