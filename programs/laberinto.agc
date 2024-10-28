.config
    VEC MASCNEG 8
.code
LABERINTO:
    # Inicializa X e IMPRP a 1 e Y a 0
    CA ZERO
    TS Y
    INCR ACC
    TS X    
    TS IMPRP
LABB:
    # Almacena los valores previos de X e Y
    CA X
    TS PREVX
    CA Y
    TS PREVY
    # Actualiza los valores de X e Y
    TC MOVIMIENTO
# Bucle de impresion
    CA ANCHOPANT
IMPR:
    TS I

    # Chequea si Y == I
    CS Y
    AD I
    EXTEND
    BZF D2
    # Si no lo es, cargamos la parte del mapa que corresponde a esta fila y la imprimimos, como siempre
    INDEX I
    CA MAPA
    INDEX I
    TS PANT

    TCF D4
D2:
    # Si lo es, hacemos un OR entre el valor del mapa y el valor que deberia tener segun la posicion en X de nustro jugador
    # Como la AGC no tiene una instruccion OR, usamos las leyes de De Morgan, que nos dicen que A OR B = NOT(NOT(A) AND NOT(B))
    INDEX I
    CS MAPA
    INDEX X
    MASK MASCNEG
    COM
    TS IMPRESION

    # Comprueba si la impresion es igual al mapa. Si esto es asi, significa que nos paramos en una pared, y nuestro movimiento es invalido
    COM
    INDEX I
    AD MAPA
    EXTEND 
    BZF D3
    # Si no es asi, simplemente imprime lo calculado o simplemente el mapa dependiendo de IMPRP. No imprimir al jugador según IMPRP
    # tiene el objetivo de hacer que este titile, para que podamos saber donde está.
    # Si IMPRP es positivo, el jugador se imprime. Si es negativo, no se imprime
    CS IMPRP
    TS IMPRP
    # Las 2 lineas anteriores invierten el signo de IMPRP
    EXTEND
    BZMF D5

    CA IMPRESION
    INDEX I
    TS PANT
    TCF D4
D5:
    INDEX I
    CA MAPA
    INDEX I
    TS PANT
    TCF D4
D3:
    # Si es asi, restablece nuestra posicion anterior
    CA PREVX
    TS X
    CA PREVY
    TS Y
    # Vuelve a imprimir
    TCF IMPR
D4:
    CCS I

    TCF IMPR

    CA X
    
    # Pequeño delay
    CA CORTO
    TS CICLOS
    TC DELAY

    # Chequea si el jugador llego a la meta. Si es asi, cambia al programa blink para indicar que ganó
    CS FINALX
    AD X
    EXTEND
    BZF D7
    TCF D6
D7:
    CS FINALY
    AD Y
    EXTEND
    BZF D8
    TCF D6
D8:
    TCF BLINK
    
D6:
    # Chequea si debe salir del programa
    CA BTN1
    EXTEND
    BZMF D1
    TC LIMPPANT
    TCF INICIO
D1:
    TCF LABB

.data
FINALX: 
    DEC 7
FINALY:
    DEC 6
MASCNEG: # Valores opuestos a los que debe tener la pantalla para cada valor de X
    DEC -1
    DEC -2
    DEC -4
    DEC -8
    DEC -16
    DEC -32
    DEC -64
    DEC -128