.config
    EXTERN FIN
.code
FOR:
    CA ANCHOPANTALLA
    EXTEND
    DIM ACC
LOOP:
    TS I

    INDEX I
    CA MAPA

    INDEX I
    TS PANTALLA

    CCS I

    TCF LOOP

    TCF FIN

.data
ANCHOPANTALLA:
    DEC 8
MAPA:
    DEC 253
    DEC 197
    DEC 145
    DEC 191
    DEC 133
    DEC 237
    DEC 129
    DEC 191