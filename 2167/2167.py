def main() -> None:
    resposta = 0
    n = int(input())
    rpms = [int(x) for x in input().strip().split(' ')]
    
    for i in range(1, n):
        if rpms[i] < rpms[i - 1]:
            resposta = i + 1
            break
    
    print(resposta)

if __name__ == "__main__":
    main()
