#include <iostream>
#include <vector>

int main(){
    int n, r, ans=0;
    std::vector<int> rpms;
    std::cin >> n;

    for (int i=0;i<n;i++){
        std::cin >> r;
        rpms.push_back(r);
    }

    for (int i=1;i<n;i++){
        if (rpms[i] < rpms[i-1]){
            ans = i + 1;
            break;
        }
    }

    std::cout << ans << std::endl;

    return 0;
}