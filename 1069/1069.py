def main() -> None:
    n: int = int(input())

    for _ in range(n):
        diamantes: int = 0
        pilha: list = []
        areia_diamantes: str = input()

        for c in areia_diamantes:
            if c == "<":
                pilha.append(c)
            elif c == ">" and pilha:
                pilha.pop()
                diamantes += 1

        print(diamantes)


if __name__ == "__main__":
    main()
