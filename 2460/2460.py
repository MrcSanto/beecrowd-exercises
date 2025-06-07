def main() -> None:
    n: int = int(input())
    fila_ingressos: list = list(map(int, input().split()))

    m: int = int(input())
    removidos: list = list(map(int, input().split()))

    for p in removidos:
        if p in fila_ingressos:
            fila_ingressos.remove(p)

    print(*fila_ingressos)


if __name__ == "__main__":
    main()
