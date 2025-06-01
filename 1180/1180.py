def main() -> None:
    n = int(input())
    valores_arr = [int(x) for x in input().strip().split(" ")[:n]]

    menor_valor = valores_arr[0]
    menor_indice = 0

    for i in range(1, n):
        if valores_arr[i] < menor_valor:
            menor_valor = valores_arr[i]
            menor_indice = i

    print(f"Menor valor: {menor_valor}")
    print(f"Posicao: {menor_indice}")


if __name__ == "__main__":
    main()
