.code
IF:
    CS LIMITE
    AD POTE
    # Las dos lineas anteriores calculan [POTE - LIMITE], que solo es positivo si POTE > LIMITE 

    EXTEND
    BZMF NOSUPERA

    CS ZERO # Carga -0 (toda la fila encendida)
    TCF IMPRIMIR

NOSUPERA:
    CA ZERO

IMPRIMIR:
    TS PANT
    
    # Pequeño delay para evitar volver inmediatamente a la pantalla de selección
    CA MEDIO
    TS CICLOS
    TC DELAY

    # Chequea si debe salir del programa
    CA BTN1
    EXTEND
    BZMF IF
    TC LIMPPANT
    TCF INICIO

.data
LIMITE: # Valor que el potenciometro debe superar para que se encienda la pantalla
    DEC 2000