def main() -> None:
    renda : float = float(input())
    taxa = 0

    salarios_corte: list[float] = [4500.00, 3000.00, 2000.00]
    aliquotas : list[float] = [0.28, 0.18, 0.08]
    
    for i, salario in enumerate(salarios_corte):
        resto = renda - salario
        if resto > 0.00:
            taxa += (resto * aliquotas[i])
            renda -= resto
            
    if taxa:
        print(f"R$ {taxa:.2f}")
    else:
        print("Isento")

if __name__ == "__main__":
    main()
