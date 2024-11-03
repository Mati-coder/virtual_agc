.code
IF:
    CS LIMITE
    AD POTE

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
    BZF IF
    TC LIMPPANT
    TCF INICIO

.data
LIMITE: # Valor que el potenciometro debe superar para que se encienda la pantalla
    DEC 2000