.code
PONG:
    # Inicializa distintas variables
    CA ZERO
    INCR ACC
    TS DIRX
    TS DIRY
    INCR ACC
    TS YP1
    TS YP2
    INCR ACC
    TS YB
    TS XB

# Bucle que evita que el juego inicie hasta que se presione BTNUP
PAUSA:
    CCS BTNUP
    TCF PONGB
    TCF PAUSA

# Bucle principal
PONGB:
    # Chequa si algun jugador debe moverse
    CA BTN1
    EXTEND
    BZF E1
    EXTEND
    DIM YP1
E1:
    CCS BTN2
    INCR YP1
    
    CA BTNLFT
    EXTEND
    BZF E2
    EXTEND
    DIM YP2
E2:
    CCS BTNRGT
    INCR YP2

    # Restablece YP1 e YP2 si se pasaron del maximo
    CS YP1
    AD MAXYP
    COM
    EXTEND
    BZMF E3
    CA MAXYP
    TS YP1
E3:
    CS YP2
    AD MAXYP
    COM
    EXTEND
    BZMF E11
    CA MAXYP
    TS YP2

    # Dependiendo de el signo de DIRX y DIRY mueve a la bola
E11:
    CA DIRX
    EXTEND
    BZMF E12
    EXTEND
    DIM XB
    TCF E13
E12:
    INCR XB
E13:
    CA DIRY
    EXTEND
    BZMF E14
    EXTEND
    DIM YB
    TCF E15
E14:
    INCR YB
E15:
    
    CA ANCHOPANT
IMPRBP:
    TS I

    # Inicializa el valor a imprimir a 0
    CA ZERO
    TS FILA

    # Si YP1 <= I <= YP1 + LENP, entonces imprime al jugador en esa fila
    CS I
    AD YP1
    EXTEND
    BZMF E4
    TCF E6
E4:
    CS I
    AD YP1
    AD LENP
    COM
    EXTEND
    BZMF E5
    TCF E6
E5:
    # Carga el valor de pantalla correspondiente al jugador 1 a la impresion
    CA MASCP1
    TS FILA
E6:
    # Si YP2 <= I <= YP2 + LENP, entonces imprime al jugador en esa fila
    CS I
    AD YP2
    EXTEND
    BZMF E7
    TCF E9
E7:
    CS I
    AD YP2
    AD LENP
    COM
    EXTEND
    BZMF E8
    TCF E9
E8:
    # Hace un OR entre FILA y el valor de pantalla correspondiente al jugador 2, para que se puedan imprimir ambos jugadores en la misma fila
    CS FILA
    MASK MASCNP2
    COM
    TS FILA
E9:
    # Si I == YB, imprime la pelota
    CS YB
    AD I
    EXTEND
    BZF E10
    # Hace un OR entre FILA y el valor de pantalla correspondiente a la pelota
    CS FILA
    INDEX XB
    MASK MASCNEG
    COM
    TS FILA
E10:
    # Imprime el valor calculado
    CA FILA
    INDEX I
    TS PANT

    CCS I
    TCF IMPRBP

    # Si YB es 0 o MAXXY, la pelota rebota, por lo que su direccion de movimiento cambai
    CA YB
    EXTEND
    BZF E16

    CS YB
    AD MAXXY
    EXTEND
    BZMF E16
    TCF E17
E16:
    # Invierte el signo de DIRY
    CS DIRY
    TS DIRY
E17:
    # Si XB es 0, se fija si YP1 <= YB <= YP2 y si es asi rebota. Si no es asi, gana el jugador 2
    CA XB
    EXTEND
    BZF E18
    TCF E22
E18:
    CS XB
    AD YP1
    EXTEND
    BZMF E19
    TCF E21
E19:
    CS XB
    AD YP1
    AD LENP
    COM
    EXTEND
    BZMF E20
    TCF E21
E20:
    CS DIRX
    TS DIRX
E21:
    # GANA EL JUGADOR 2

    CA LARGO
    TS CICLOS
    TC DELAY

    CA LARGO
    TS CICLOS
    TC DELAY

    TCF BLINK
E22:
    # Si XB es MAXXY, se fija si YP2 <= YB <= YP2 y si es asi rebota. Si no es asi, gana el jugador 1
    CS XB
    AD MAXXY
    EXTEND
    BZMF E23
    TCF E27
E23:
    CS XB
    AD YP2
    EXTEND
    BZMF E24
    TCF E26
E24:
    CS XB
    AD YP2
    AD LENP
    COM
    EXTEND
    BZMF E25
    TCF E26
E25:
    CS DIRX
    TS DIRX
E26:
    # GANA EL JUGADOR 1

    CA LARGO
    TS CICLOS
    TC DELAY

    CA LARGO
    TS CICLOS
    TC DELAY

    TCF BLINK
E27:
    # Chequea si debe salir del programa
    CA BTNDWN
    EXTEND
    BZF PONGB 
    TC LIMPPANT
    TCF INICIO

.data
LENP: # Alto del jugador de pong
    DEC 2
MAXYP: # Maximo valor Y del jugador
    DEC 5
MASCP1: # Valor del jugador 1 de pong
    DEC 1
MASCNP2: # Negativo del valor del jugador 2 de pong
    DEC -128