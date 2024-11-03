.config
    VEC MASCARAS 8
.code
PLAYER:
    # Imprime la posicion inicial del jugador inmediatamente, para que sepamos que entramos a este modo
    CA MASC
    TS PANT
    # Pequeño delay para evitar volver inmediatamente a la pantalla de selección
    CA MEDIO
    TS CICLOS
    TC DELAY
    # Inicializa X e Y a 0
    CA ZERO
    TS X
    TS Y
# Bucle principal
PLAYERB:
    # Carga el valor de fila que corresponda segun la posicion en X
    INDEX X
    CA MASC

    # Lo imprime en la fila correspondiente segun la posicion en Y
    INDEX Y
    TS PANT

    # Guarda el valor de Y
    CA Y
    TS PREVY

    # Actualiza X e Y segun los botones pulsados
    TC MOVIMIENTO

    # Pequeño delay
    CA CORTO
    TS CICLOS
    TC DELAY

    # Carga un 0 en la fila impresa en el bucle anterior, ya que es lo unico impreso en toda la pantalla
    # Esto es mas eficiente que cargar un 0 en todas las filas, porque sabemos ya son todas 0 excepto una
    CA ZERO
    INDEX PREVY
    TS PANT

    # Chequea si debe salir del programa
    CA BTN1
    EXTEND
    BZF C1
    TC LIMPPANT
    TCF INICIO
C1:
    TCF PLAYERB

.data
MASC: # Lista que almacena los valores de la pantalla para cada valor de X del jugador
    DEC 1
    DEC 2
    DEC 4
    DEC 8
    DEC 16
    DEC 32
    DEC 64
    DEC 128