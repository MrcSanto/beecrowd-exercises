def main() -> None:
    p, n = (int(x) for x in input().strip().split(' '))
    altura_canos = [int(x) for x in input().strip().split(' ')]
    
    for i in range(1, n):
        cano_atual = altura_canos[i-1]
        cano_a_pular = altura_canos[i]
        diff = abs(cano_atual - cano_a_pular)
        if diff > p:
            print("GAME OVER")
            return
        
    print("YOU WIN")

if __name__ == "__main__":
    main()
