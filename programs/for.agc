.config
    VEC MAPA 8
.code
# Ejemplo de un bucle for simple. Imprime una imagen en la pantalla (un laberinto)
FOR: 
    CA ANCHOPANT
BUCLEF:
    TS I

    INDEX I
    CA MAPA

    INDEX I
    TS PANT

    CCS I

    TCF BUCLEF

    # Pequeño delay para evitar volver inmediatamente a la pantalla de selección
    CA LARGO
    TS CICLOS
    TC DELAY

    TC LIMPPANT

    # Chequea si debe salir del programa
    CA BTN1
    EXTEND
    BZF FOR
    TCF INICIO

.data
MAPA: # Laberinto
    DEC 253
    DEC 197
    DEC 145
    DEC 191
    DEC 133
    DEC 237
    DEC 129
    DEC 191