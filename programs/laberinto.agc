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

# Imprime todo el mapa
    CA ANCHOPANT
IMPRMAP:
    TS I

    INDEX I
    CA MAPA
    INDEX I
    TS PANT
    
    CCS I
    TCF IMPRMAP

LABB:
    # Almacena los valores previos de X e Y
    CA X
    TS PREVX
    CA Y
    TS PREVY
    # Actualiza los valores de X e Y
    TC MOVIMIENTO

# Actualiza solo la fila correspondiente a la posicion del jugador
IMPR:
    # Hacemos un OR entre el valor del mapa y el valor que deberia tener segun la posicion en X de nustro jugador
    # Como la AGC no tiene una instruccion OR, usamos las leyes de De Morgan, que nos dicen que A OR B = NOT(NOT(A) AND NOT(B))
    INDEX Y
    CS MAPA
    INDEX X
    MASK MASCNEG
    TS NOTIMPR

    # Comprueba si la impresion es igual al mapa. Si esto es asi, significa que nos paramos en una pared, y nuestro movimiento es invalido
    INDEX Y
    AD MAPA
    EXTEND 
    BZF D3

    CS NOTIMPR
    INDEX Y
    TS PANT
    TCF D4
D3:
    # Restablece nuestra posicion anterior
    CA PREVX
    TS X
    CA PREVY
    TS Y
    # Vuelve a imprimir
    TCF IMPR
D4:
    # Pequeño delay
    CA CORTO
    TS CICLOS
    TC DELAY

    # Restablece las filas donde estuvimos anteriormente, para evitar que el jugador se mantenga impreso luego de moverse. Tiene ademas el efecto de hacer parpadear al jugador
    INDEX PREVY
    CA MAPA
    INDEX PREVY
    TS PANT

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
    BZF D1
    TC LIMPPANT
    TCF INICIO
D1:
    TCF LABB

.data
FINALX: 
    DEC 6
FINALY:
    DEC 7
MASCNEG: # Valores opuestos a los que debe tener la pantalla para cada valor de X
    DEC -1
    DEC -2
    DEC -4
    DEC -8
    DEC -16
    DEC -32
    DEC -64
    DEC -128